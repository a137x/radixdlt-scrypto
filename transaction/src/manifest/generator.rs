use sbor::any::{encode_any, Value};
use sbor::rust::collections::BTreeSet;
use sbor::rust::collections::HashMap;
use sbor::rust::str::FromStr;
use sbor::type_id::*;
use scrypto::abi::*;
use scrypto::address::Bech32Decoder;
use scrypto::engine::types::*;
use scrypto::values::*;
use scrypto::vec_to_struct;

use crate::errors::*;
use crate::manifest::ast;
use crate::model::*;
use crate::validation::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorError {
    InvalidType {
        expected_type: ast::Type,
        actual: ast::Type,
    },
    InvalidValue {
        expected_type: Vec<ast::Type>,
        actual: ast::Value,
    },
    InvalidPackageAddress(String),
    InvalidComponentAddress(String),
    InvalidResourceAddress(String),
    InvalidDecimal(String),
    InvalidHash(String),
    InvalidKeyValueStoreId(String),
    InvalidVaultId(String),
    InvalidNonFungibleId(String),
    InvalidNonFungibleAddress(String),
    OddNumberOfElements(usize),
    NameResolverError(NameResolverError),
    IdValidationError(IdValidationError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameResolverError {
    UndefinedBucket(String),
    UndefinedProof(String),
    NamedAlreadyDefined(String),
}

pub struct NameResolver {
    named_buckets: HashMap<String, BucketId>,
    named_proofs: HashMap<String, ProofId>,
}

impl NameResolver {
    pub fn new() -> Self {
        Self {
            named_buckets: HashMap::new(),
            named_proofs: HashMap::new(),
        }
    }

    pub fn insert_bucket(
        &mut self,
        name: String,
        bucket_id: BucketId,
    ) -> Result<(), NameResolverError> {
        if self.named_buckets.contains_key(&name) || self.named_proofs.contains_key(&name) {
            Err(NameResolverError::NamedAlreadyDefined(name))
        } else {
            self.named_buckets.insert(name, bucket_id);
            Ok(())
        }
    }

    pub fn insert_proof(
        &mut self,
        name: String,
        proof_id: ProofId,
    ) -> Result<(), NameResolverError> {
        if self.named_buckets.contains_key(&name) || self.named_proofs.contains_key(&name) {
            Err(NameResolverError::NamedAlreadyDefined(name))
        } else {
            self.named_proofs.insert(name, proof_id);
            Ok(())
        }
    }

    pub fn resolve_bucket(&mut self, name: &str) -> Result<BucketId, NameResolverError> {
        match self.named_buckets.get(name).cloned() {
            Some(bucket_id) => Ok(bucket_id),
            None => Err(NameResolverError::UndefinedBucket(name.into())),
        }
    }

    pub fn resolve_proof(&mut self, name: &str) -> Result<ProofId, NameResolverError> {
        match self.named_proofs.get(name).cloned() {
            Some(proof_id) => Ok(proof_id),
            None => Err(NameResolverError::UndefinedProof(name.into())),
        }
    }
}

pub fn generate_manifest(
    instructions: &[ast::Instruction],
    bech32_decoder: &Bech32Decoder,
) -> Result<TransactionManifest, GeneratorError> {
    let mut id_validator = IdValidator::new();
    let mut name_resolver = NameResolver::new();
    let mut output = Vec::new();

    for instruction in instructions {
        output.push(generate_instruction(
            instruction,
            &mut id_validator,
            &mut name_resolver,
            bech32_decoder,
        )?);
    }

    Ok(TransactionManifest {
        instructions: output,
    })
}

pub fn generate_instruction(
    instruction: &ast::Instruction,
    id_validator: &mut IdValidator,
    resolver: &mut NameResolver,
    bech32_decoder: &Bech32Decoder,
) -> Result<Instruction, GeneratorError> {
    Ok(match instruction {
        ast::Instruction::TakeFromWorktop {
            resource_address,
            new_bucket,
        } => {
            let bucket_id = id_validator
                .new_bucket()
                .map_err(GeneratorError::IdValidationError)?;
            declare_bucket(new_bucket, resolver, bucket_id)?;

            Instruction::TakeFromWorktop {
                resource_address: generate_resource_address(resource_address, bech32_decoder)?,
            }
        }
        ast::Instruction::TakeFromWorktopByAmount {
            amount,
            resource_address,
            new_bucket,
        } => {
            let bucket_id = id_validator
                .new_bucket()
                .map_err(GeneratorError::IdValidationError)?;
            declare_bucket(new_bucket, resolver, bucket_id)?;

            Instruction::TakeFromWorktopByAmount {
                amount: generate_decimal(amount)?,
                resource_address: generate_resource_address(resource_address, bech32_decoder)?,
            }
        }
        ast::Instruction::TakeFromWorktopByIds {
            ids,
            resource_address,
            new_bucket,
        } => {
            let bucket_id = id_validator
                .new_bucket()
                .map_err(GeneratorError::IdValidationError)?;
            declare_bucket(new_bucket, resolver, bucket_id)?;

            Instruction::TakeFromWorktopByIds {
                ids: generate_non_fungible_ids(ids)?,
                resource_address: generate_resource_address(resource_address, bech32_decoder)?,
            }
        }
        ast::Instruction::ReturnToWorktop { bucket } => {
            let bucket_id = generate_bucket(bucket, resolver)?;
            id_validator
                .drop_bucket(bucket_id)
                .map_err(GeneratorError::IdValidationError)?;
            Instruction::ReturnToWorktop { bucket_id }
        }
        ast::Instruction::AssertWorktopContains { resource_address } => {
            Instruction::AssertWorktopContains {
                resource_address: generate_resource_address(resource_address, bech32_decoder)?,
            }
        }
        ast::Instruction::AssertWorktopContainsByAmount {
            amount,
            resource_address,
        } => Instruction::AssertWorktopContainsByAmount {
            amount: generate_decimal(amount)?,
            resource_address: generate_resource_address(resource_address, bech32_decoder)?,
        },
        ast::Instruction::AssertWorktopContainsByIds {
            ids,
            resource_address,
        } => Instruction::AssertWorktopContainsByIds {
            ids: generate_non_fungible_ids(ids)?,
            resource_address: generate_resource_address(resource_address, bech32_decoder)?,
        },
        ast::Instruction::PopFromAuthZone { new_proof } => {
            let proof_id = id_validator
                .new_proof(ProofKind::AuthZoneProof)
                .map_err(GeneratorError::IdValidationError)?;
            declare_proof(new_proof, resolver, proof_id)?;

            Instruction::PopFromAuthZone
        }
        ast::Instruction::PushToAuthZone { proof } => {
            let proof_id = generate_proof(proof, resolver)?;
            id_validator
                .drop_proof(proof_id)
                .map_err(GeneratorError::IdValidationError)?;
            Instruction::PushToAuthZone { proof_id }
        }
        ast::Instruction::ClearAuthZone => Instruction::ClearAuthZone,

        ast::Instruction::CreateProofFromAuthZone {
            resource_address,
            new_proof,
        } => {
            let resource_address = generate_resource_address(resource_address, bech32_decoder)?;
            let proof_id = id_validator
                .new_proof(ProofKind::AuthZoneProof)
                .map_err(GeneratorError::IdValidationError)?;
            declare_proof(new_proof, resolver, proof_id)?;

            Instruction::CreateProofFromAuthZone { resource_address }
        }
        ast::Instruction::CreateProofFromAuthZoneByAmount {
            amount,
            resource_address,
            new_proof,
        } => {
            let amount = generate_decimal(amount)?;
            let resource_address = generate_resource_address(resource_address, bech32_decoder)?;
            let proof_id = id_validator
                .new_proof(ProofKind::AuthZoneProof)
                .map_err(GeneratorError::IdValidationError)?;
            declare_proof(new_proof, resolver, proof_id)?;

            Instruction::CreateProofFromAuthZoneByAmount {
                amount,
                resource_address,
            }
        }
        ast::Instruction::CreateProofFromAuthZoneByIds {
            ids,
            resource_address,
            new_proof,
        } => {
            let ids = generate_non_fungible_ids(ids)?;
            let resource_address = generate_resource_address(resource_address, bech32_decoder)?;
            let proof_id = id_validator
                .new_proof(ProofKind::AuthZoneProof)
                .map_err(GeneratorError::IdValidationError)?;
            declare_proof(new_proof, resolver, proof_id)?;

            Instruction::CreateProofFromAuthZoneByIds {
                ids,
                resource_address,
            }
        }
        ast::Instruction::CreateProofFromBucket { bucket, new_proof } => {
            let bucket_id = generate_bucket(bucket, resolver)?;
            let proof_id = id_validator
                .new_proof(ProofKind::BucketProof(bucket_id))
                .map_err(GeneratorError::IdValidationError)?;
            declare_proof(new_proof, resolver, proof_id)?;

            Instruction::CreateProofFromBucket { bucket_id }
        }
        ast::Instruction::CloneProof { proof, new_proof } => {
            let proof_id = generate_proof(proof, resolver)?;
            let proof_id2 = id_validator
                .clone_proof(proof_id)
                .map_err(GeneratorError::IdValidationError)?;
            declare_proof(new_proof, resolver, proof_id2)?;

            Instruction::CloneProof { proof_id }
        }
        ast::Instruction::DropProof { proof } => {
            let proof_id = generate_proof(proof, resolver)?;
            id_validator
                .drop_proof(proof_id)
                .map_err(GeneratorError::IdValidationError)?;
            Instruction::DropProof { proof_id }
        }
        ast::Instruction::DropAllProofs => {
            id_validator
                .drop_all_proofs()
                .map_err(GeneratorError::IdValidationError)?;
            Instruction::DropAllProofs
        }
        ast::Instruction::CallFunction {
            package_address,
            blueprint_name,
            function,
            args,
        } => {
            let args = generate_args(args, resolver, bech32_decoder)?;
            let mut fields = Vec::new();
            for arg in &args {
                let validated_arg = ScryptoValue::from_slice(arg).unwrap();
                id_validator
                    .move_resources(&validated_arg)
                    .map_err(GeneratorError::IdValidationError)?;
                fields.push(validated_arg.dom);
            }

            Instruction::CallFunction {
                package_address: generate_package_address(package_address, bech32_decoder)?,
                blueprint_name: generate_string(blueprint_name)?,
                method_name: generate_string(function)?,
                arg: vec_to_struct!(fields),
            }
        }
        ast::Instruction::CallMethod {
            component_address,
            method,
            args,
        } => {
            let args = generate_args(args, resolver, bech32_decoder)?;
            let mut fields = Vec::new();
            for arg in &args {
                let validated_arg = ScryptoValue::from_slice(arg).unwrap();
                id_validator
                    .move_resources(&validated_arg)
                    .map_err(GeneratorError::IdValidationError)?;
                fields.push(validated_arg.dom);
            }

            Instruction::CallMethod {
                component_address: generate_component_address(component_address, bech32_decoder)?,
                method_name: generate_string(method)?,
                arg: vec_to_struct!(fields),
            }
        }
        ast::Instruction::CallMethodWithAllResources {
            component_address,
            method,
        } => {
            id_validator
                .move_all_buckets()
                .map_err(GeneratorError::IdValidationError)?;
            Instruction::CallMethodWithAllResources {
                component_address: generate_component_address(component_address, bech32_decoder)?,
                method: generate_string(method)?,
            }
        }
        ast::Instruction::PublishPackage { package } => Instruction::PublishPackage {
            package: generate_bytes(package)?,
        },
    })
}

#[macro_export]
macro_rules! invalid_type {
    ( $v:expr, $($exp:expr),+ ) => {
        Err(GeneratorError::InvalidValue {
            expected_type: vec!($($exp),+),
            actual: $v.clone(),
        })
    };
}

fn generate_args(
    values: &Vec<ast::Value>,
    resolver: &mut NameResolver,
    bech32_decoder: &Bech32Decoder,
) -> Result<Vec<Vec<u8>>, GeneratorError> {
    let mut result = Vec::new();
    for v in values {
        let value = generate_value(v, None, resolver, bech32_decoder)?;

        result.push(encode_any(&value));
    }
    Ok(result)
}

fn generate_string(value: &ast::Value) -> Result<String, GeneratorError> {
    match value {
        ast::Value::String(s) => Ok(s.into()),
        v @ _ => invalid_type!(v, ast::Type::String),
    }
}

fn generate_bytes(value: &ast::Value) -> Result<Vec<u8>, GeneratorError> {
    match value {
        ast::Value::Bytes(bytes) => Ok(bytes.clone()),
        ast::Value::List(ty, values) => {
            if ty == &ast::Type::U8 {
                let mut result = Vec::new();
                for v in values {
                    match v {
                        ast::Value::U8(num) => {
                            result.push(*num);
                        }
                        _ => {
                            return Err(GeneratorError::InvalidValue {
                                expected_type: vec![ast::Type::U8],
                                actual: v.clone(),
                            })
                        }
                    }
                }
                Ok(result)
            } else {
                Err(GeneratorError::InvalidType {
                    expected_type: ast::Type::U8,
                    actual: *ty,
                })
            }
        }
        v @ _ => invalid_type!(v, ast::Type::List, ast::Type::Bytes),
    }
}

fn generate_decimal(value: &ast::Value) -> Result<Decimal, GeneratorError> {
    match value {
        ast::Value::Decimal(inner) => match &**inner {
            ast::Value::String(s) => {
                Decimal::from_str(s).map_err(|_| GeneratorError::InvalidDecimal(s.into()))
            }
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::Decimal),
    }
}

fn generate_package_address(
    value: &ast::Value,
    bech32_decoder: &Bech32Decoder,
) -> Result<PackageAddress, GeneratorError> {
    match value {
        ast::Value::PackageAddress(inner) => match &**inner {
            ast::Value::String(s) => bech32_decoder
                .validate_and_decode_package_address(s)
                .map_err(|_| GeneratorError::InvalidPackageAddress(s.into())),
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::PackageAddress),
    }
}

fn generate_component_address(
    value: &ast::Value,
    bech32_decoder: &Bech32Decoder,
) -> Result<ComponentAddress, GeneratorError> {
    match value {
        ast::Value::ComponentAddress(inner) => match &**inner {
            ast::Value::String(s) => bech32_decoder
                .validate_and_decode_component_address(s)
                .map_err(|_| GeneratorError::InvalidComponentAddress(s.into())),
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::ComponentAddress),
    }
}

fn generate_resource_address(
    value: &ast::Value,
    bech32_decoder: &Bech32Decoder,
) -> Result<ResourceAddress, GeneratorError> {
    match value {
        ast::Value::ResourceAddress(inner) => match &**inner {
            ast::Value::String(s) => bech32_decoder
                .validate_and_decode_resource_address(s)
                .map_err(|_| GeneratorError::InvalidResourceAddress(s.into())),
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::ResourceAddress),
    }
}

fn generate_hash(value: &ast::Value) -> Result<Hash, GeneratorError> {
    match value {
        ast::Value::Hash(inner) => match &**inner {
            ast::Value::String(s) => {
                Hash::from_str(s).map_err(|_| GeneratorError::InvalidHash(s.into()))
            }
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::Hash),
    }
}

fn declare_bucket(
    value: &ast::Value,
    resolver: &mut NameResolver,
    bucket_id: BucketId,
) -> Result<(), GeneratorError> {
    match value {
        ast::Value::Bucket(inner) => match &**inner {
            ast::Value::String(name) => resolver
                .insert_bucket(name.to_string(), bucket_id)
                .map_err(GeneratorError::NameResolverError),
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::Bucket),
    }
}

fn generate_bucket(
    value: &ast::Value,
    resolver: &mut NameResolver,
) -> Result<BucketId, GeneratorError> {
    match value {
        ast::Value::Bucket(inner) => match &**inner {
            ast::Value::U32(n) => Ok(*n),
            ast::Value::String(s) => resolver
                .resolve_bucket(&s)
                .map_err(GeneratorError::NameResolverError),
            v @ _ => invalid_type!(v, ast::Type::U32, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::Bucket),
    }
}

fn declare_proof(
    value: &ast::Value,
    resolver: &mut NameResolver,
    proof_id: ProofId,
) -> Result<(), GeneratorError> {
    match value {
        ast::Value::Proof(inner) => match &**inner {
            ast::Value::String(name) => resolver
                .insert_proof(name.to_string(), proof_id)
                .map_err(GeneratorError::NameResolverError),
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::Proof),
    }
}

fn generate_proof(
    value: &ast::Value,
    resolver: &mut NameResolver,
) -> Result<ProofId, GeneratorError> {
    match value {
        ast::Value::Proof(inner) => match &**inner {
            ast::Value::U32(n) => Ok(*n),
            ast::Value::String(s) => resolver
                .resolve_proof(&s)
                .map_err(GeneratorError::NameResolverError),
            v @ _ => invalid_type!(v, ast::Type::U32, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::Proof),
    }
}

fn generate_non_fungible_id(value: &ast::Value) -> Result<NonFungibleId, GeneratorError> {
    match value {
        ast::Value::NonFungibleId(inner) => match &**inner {
            ast::Value::String(s) => NonFungibleId::from_str(s)
                .map_err(|_| GeneratorError::InvalidNonFungibleId(s.into())),
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::NonFungibleId),
    }
}

fn generate_non_fungible_address(value: &ast::Value) -> Result<NonFungibleAddress, GeneratorError> {
    match value {
        ast::Value::NonFungibleAddress(inner) => match &**inner {
            ast::Value::String(s) => NonFungibleAddress::from_str(s)
                .map_err(|_| GeneratorError::InvalidNonFungibleAddress(s.into())),
            v @ _ => invalid_type!(v, ast::Type::String),
        },
        v @ _ => invalid_type!(v, ast::Type::NonFungibleAddress),
    }
}

fn generate_non_fungible_ids(
    value: &ast::Value,
) -> Result<BTreeSet<NonFungibleId>, GeneratorError> {
    match value {
        ast::Value::Set(kind, values) => {
            if kind != &ast::Type::NonFungibleId {
                return Err(GeneratorError::InvalidType {
                    expected_type: ast::Type::String,
                    actual: kind.clone(),
                });
            }

            values.iter().map(|v| generate_non_fungible_id(v)).collect()
        }
        v @ _ => invalid_type!(v, ast::Type::Set),
    }
}

fn generate_value(
    value: &ast::Value,
    expected: Option<ast::Type>,
    resolver: &mut NameResolver,
    bech32_decoder: &Bech32Decoder,
) -> Result<Value, GeneratorError> {
    if let Some(ty) = expected {
        if ty != value.kind() {
            return Err(GeneratorError::InvalidValue {
                expected_type: vec![ty],
                actual: value.clone(),
            });
        }
    }

    match value {
        ast::Value::Unit => Ok(Value::Unit),
        ast::Value::Bool(value) => Ok(Value::Bool { value: *value }),
        ast::Value::I8(value) => Ok(Value::I8 { value: *value }),
        ast::Value::I16(value) => Ok(Value::I16 { value: *value }),
        ast::Value::I32(value) => Ok(Value::I32 { value: *value }),
        ast::Value::I64(value) => Ok(Value::I64 { value: *value }),
        ast::Value::I128(value) => Ok(Value::I128 { value: *value }),
        ast::Value::U8(value) => Ok(Value::U8 { value: *value }),
        ast::Value::U16(value) => Ok(Value::U16 { value: *value }),
        ast::Value::U32(value) => Ok(Value::U32 { value: *value }),
        ast::Value::U64(value) => Ok(Value::U64 { value: *value }),
        ast::Value::U128(value) => Ok(Value::U128 { value: *value }),
        ast::Value::String(value) => Ok(Value::String {
            value: value.clone(),
        }),
        ast::Value::Struct(fields) => Ok(Value::Struct {
            fields: generate_singletons(fields, None, resolver, bech32_decoder)?,
        }),
        ast::Value::Enum(name, fields) => Ok(Value::Enum {
            name: name.clone(),
            fields: generate_singletons(fields, None, resolver, bech32_decoder)?,
        }),
        ast::Value::Option(value) => match &**value {
            Some(inner) => Ok(Value::Option {
                value: Some(generate_value(inner, None, resolver, bech32_decoder)?).into(),
            }),
            None => Ok(Value::Option { value: None.into() }),
        },
        ast::Value::Array(element_type, elements) => Ok(Value::Array {
            element_type_id: generate_type_id(element_type),
            elements: generate_singletons(elements, Some(*element_type), resolver, bech32_decoder)?,
        }),
        ast::Value::Tuple(elements) => Ok(Value::Tuple {
            elements: generate_singletons(elements, None, resolver, bech32_decoder)?,
        }),
        ast::Value::Result(value) => match &**value {
            Ok(inner) => Ok(Value::Result {
                value: Ok(generate_value(inner, None, resolver, bech32_decoder)?).into(),
            }),
            Err(inner) => Ok(Value::Result {
                value: Err(generate_value(inner, None, resolver, bech32_decoder)?).into(),
            }),
        },
        ast::Value::List(element_type, elements) => Ok(Value::List {
            element_type_id: generate_type_id(element_type),
            elements: generate_singletons(elements, Some(*element_type), resolver, bech32_decoder)?,
        }),
        ast::Value::Set(element_type, elements) => Ok(Value::Set {
            element_type_id: generate_type_id(element_type),
            elements: generate_singletons(elements, Some(*element_type), resolver, bech32_decoder)?,
        }),
        ast::Value::Map(key_type, value_type, elements) => Ok(Value::Map {
            key_type_id: generate_type_id(key_type),
            value_type_id: generate_type_id(value_type),
            elements: generate_pairs(elements, *key_type, *value_type, resolver, bech32_decoder)?,
        }),
        ast::Value::Decimal(_) => generate_decimal(value).map(|v| Value::Custom {
            type_id: ScryptoType::Decimal.id(),
            bytes: v.to_vec(),
        }),
        ast::Value::PackageAddress(_) => {
            generate_package_address(value, bech32_decoder).map(|v| Value::Custom {
                type_id: ScryptoType::PackageAddress.id(),
                bytes: v.to_vec(),
            })
        }
        ast::Value::ComponentAddress(_) => {
            generate_component_address(value, bech32_decoder).map(|v| Value::Custom {
                type_id: ScryptoType::ComponentAddress.id(),
                bytes: v.to_vec(),
            })
        }
        ast::Value::ResourceAddress(_) => {
            generate_resource_address(value, bech32_decoder).map(|v| Value::Custom {
                type_id: ScryptoType::ResourceAddress.id(),
                bytes: v.to_vec(),
            })
        }
        ast::Value::Hash(_) => generate_hash(value).map(|v| Value::Custom {
            type_id: ScryptoType::Hash.id(),
            bytes: v.to_vec(),
        }),
        ast::Value::Bucket(_) => generate_bucket(value, resolver).map(|v| Value::Custom {
            type_id: ScryptoType::Bucket.id(),
            bytes: scrypto::resource::Bucket(v).to_vec(),
        }),
        ast::Value::Proof(_) => generate_proof(value, resolver).map(|v| Value::Custom {
            type_id: ScryptoType::Proof.id(),
            bytes: scrypto::resource::Proof(v).to_vec(),
        }),
        ast::Value::NonFungibleId(_) => generate_non_fungible_id(value).map(|v| Value::Custom {
            type_id: ScryptoType::NonFungibleId.id(),
            bytes: v.to_vec(),
        }),
        ast::Value::NonFungibleAddress(_) => {
            generate_non_fungible_address(value).map(|v| Value::Custom {
                type_id: ScryptoType::NonFungibleAddress.id(),
                bytes: v.to_vec(),
            })
        }
        ast::Value::Bytes(_) => match value {
            ast::Value::Bytes(bytes) => {
                let mut elements = Vec::new();
                for b in bytes {
                    elements.push(Value::U8 { value: *b });
                }
                Ok(Value::List {
                    element_type_id: TYPE_U8,
                    elements,
                })
            }
            v @ _ => invalid_type!(v, ast::Type::Bytes),
        },
    }
}

fn generate_singletons(
    elements: &Vec<ast::Value>,
    ty: Option<ast::Type>,
    resolver: &mut NameResolver,
    bech32_decoder: &Bech32Decoder,
) -> Result<Vec<Value>, GeneratorError> {
    let mut result = vec![];
    for element in elements {
        result.push(generate_value(element, ty, resolver, bech32_decoder)?);
    }
    Ok(result)
}

fn generate_pairs(
    elements: &Vec<ast::Value>,
    key_type: ast::Type,
    value_type: ast::Type,
    resolver: &mut NameResolver,
    bech32_decoder: &Bech32Decoder,
) -> Result<Vec<Value>, GeneratorError> {
    if elements.len() % 2 != 0 {
        return Err(GeneratorError::OddNumberOfElements(elements.len()));
    }
    let mut result = vec![];
    for i in 0..elements.len() / 2 {
        result.push(generate_value(
            &elements[2 * i],
            Some(key_type),
            resolver,
            bech32_decoder,
        )?);
        result.push(generate_value(
            &elements[2 * i + 1],
            Some(value_type),
            resolver,
            bech32_decoder,
        )?);
    }
    Ok(result)
}

fn generate_type_id(ty: &ast::Type) -> u8 {
    match ty {
        ast::Type::Unit => TYPE_UNIT,
        ast::Type::Bool => TYPE_BOOL,
        ast::Type::I8 => TYPE_I8,
        ast::Type::I16 => TYPE_I16,
        ast::Type::I32 => TYPE_I32,
        ast::Type::I64 => TYPE_I64,
        ast::Type::I128 => TYPE_I128,
        ast::Type::U8 => TYPE_U8,
        ast::Type::U16 => TYPE_U16,
        ast::Type::U32 => TYPE_U32,
        ast::Type::U64 => TYPE_U64,
        ast::Type::U128 => TYPE_U128,
        ast::Type::String => TYPE_STRING,
        ast::Type::Struct => TYPE_STRUCT,
        ast::Type::Enum => TYPE_ENUM,
        ast::Type::Option => TYPE_OPTION,
        ast::Type::Array => TYPE_ARRAY,
        ast::Type::Tuple => TYPE_TUPLE,
        ast::Type::Result => TYPE_RESULT,
        ast::Type::List => TYPE_LIST,
        ast::Type::Set => TYPE_SET,
        ast::Type::Map => TYPE_MAP,
        ast::Type::Decimal => ScryptoType::Decimal.id(),
        ast::Type::PackageAddress => ScryptoType::PackageAddress.id(),
        ast::Type::ComponentAddress => ScryptoType::ComponentAddress.id(),
        ast::Type::ResourceAddress => ScryptoType::ResourceAddress.id(),
        ast::Type::Hash => ScryptoType::Hash.id(),
        ast::Type::Bucket => ScryptoType::Bucket.id(),
        ast::Type::Proof => ScryptoType::Proof.id(),
        ast::Type::NonFungibleId => ScryptoType::NonFungibleId.id(),
        ast::Type::NonFungibleAddress => ScryptoType::NonFungibleAddress.id(),
        ast::Type::Bytes => TYPE_LIST,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::lexer::tokenize;
    use crate::manifest::parser::Parser;
    use scrypto::address::Bech32Decoder;
    use scrypto::buffer::scrypto_encode;
    use scrypto::core::Network;
    use scrypto::prelude::Package;
    use scrypto::to_struct;

    #[macro_export]
    macro_rules! generate_value_ok {
        ( $s:expr, $expected:expr ) => {{
            let value = Parser::new(tokenize($s).unwrap()).parse_value().unwrap();
            let mut resolver = NameResolver::new();
            assert_eq!(
                generate_value(
                    &value,
                    None,
                    &mut resolver,
                    &Bech32Decoder::new_from_network(&Network::LocalSimulator)
                ),
                Ok($expected)
            );
        }};
    }

    #[macro_export]
    macro_rules! generate_instruction_ok {
        ( $s:expr, $expected:expr ) => {{
            let instruction = Parser::new(tokenize($s).unwrap())
                .parse_instruction()
                .unwrap();
            let mut id_validator = IdValidator::new();
            let mut resolver = NameResolver::new();
            assert_eq!(
                generate_instruction(
                    &instruction,
                    &mut id_validator,
                    &mut resolver,
                    &Bech32Decoder::new_from_network(&Network::LocalSimulator)
                ),
                Ok($expected)
            );
        }};
    }

    #[macro_export]
    macro_rules! generate_value_error {
        ( $s:expr, $expected:expr ) => {{
            let value = Parser::new(tokenize($s).unwrap()).parse_value().unwrap();
            match generate_value(
                &value,
                None,
                &mut NameResolver::new(),
                &Bech32Decoder::new_from_network(&Network::LocalSimulator),
            ) {
                Ok(_) => {
                    panic!("Expected {:?} but no error is thrown", $expected);
                }
                Err(e) => {
                    assert_eq!(e, $expected);
                }
            }
        }};
    }

    #[test]
    fn test_value() {
        generate_value_ok!(r#"()"#, Value::Unit);
        generate_value_ok!(r#"true"#, Value::Bool { value: true });
        generate_value_ok!(r#"false"#, Value::Bool { value: false });
        generate_value_ok!(r#"1i8"#, Value::I8 { value: 1 });
        generate_value_ok!(r#"1i128"#, Value::I128 { value: 1 });
        generate_value_ok!(r#"1u8"#, Value::U8 { value: 1 });
        generate_value_ok!(r#"1u128"#, Value::U128 { value: 1 });
        generate_value_ok!(
            r#"Struct(Bucket(1u32), Proof(2u32), "bar")"#,
            Value::Struct {
                fields: vec![
                    Value::Custom {
                        type_id: ScryptoType::Bucket.id(),
                        bytes: scrypto::resource::Bucket(1).to_vec()
                    },
                    Value::Custom {
                        type_id: ScryptoType::Proof.id(),
                        bytes: scrypto::resource::Proof(2).to_vec()
                    },
                    Value::String {
                        value: "bar".into()
                    }
                ]
            }
        );
        generate_value_ok!(
            r#"Struct(Decimal("1.0"), Hash("aa37f5a71083a9aa044fb936678bfd74f848e930d2de482a49a73540ea72aa5c"))"#,
            Value::Struct {
                fields: vec![
                    Value::Custom {
                        type_id: ScryptoType::Decimal.id(),
                        bytes: Decimal::from_str("1.0").unwrap().to_vec()
                    },
                    Value::Custom {
                        type_id: ScryptoType::Hash.id(),
                        bytes: Hash::from_str(
                            "aa37f5a71083a9aa044fb936678bfd74f848e930d2de482a49a73540ea72aa5c"
                        )
                        .unwrap()
                        .to_vec()
                    },
                ]
            }
        );
        generate_value_ok!(r#"Struct()"#, Value::Struct { fields: vec![] });
        generate_value_ok!(
            r#"Enum("Variant", "abc")"#,
            Value::Enum {
                name: "Variant".to_string(),
                fields: vec![Value::String {
                    value: "abc".to_owned()
                }]
            }
        );
        generate_value_ok!(
            r#"Enum("Variant")"#,
            Value::Enum {
                name: "Variant".to_string(),
                fields: vec![]
            }
        );
        generate_value_ok!(
            r#"Array<Option>(Some(1u64), None)"#,
            Value::Array {
                element_type_id: TYPE_OPTION,
                elements: vec![
                    Value::Option {
                        value: Some(Value::U64 { value: 1 }).into()
                    },
                    Value::Option { value: None.into() }
                ]
            }
        );
        generate_value_ok!(
            r#"Tuple(Ok(1u64), Err(2u64))"#,
            Value::Tuple {
                elements: vec![
                    Value::Result {
                        value: Ok(Value::U64 { value: 1 }).into()
                    },
                    Value::Result {
                        value: Err(Value::U64 { value: 2 }).into()
                    },
                ]
            }
        );
        generate_value_ok!(
            r#"Map<Set, List>(Set<U8>(1u8), List<U8>(2u8))"#,
            Value::Map {
                key_type_id: TYPE_SET,
                value_type_id: TYPE_LIST,
                elements: vec![
                    Value::Set {
                        element_type_id: TYPE_U8,
                        elements: vec![Value::U8 { value: 1 }]
                    },
                    Value::List {
                        element_type_id: TYPE_U8,
                        elements: vec![Value::U8 { value: 2 }]
                    },
                ]
            }
        );
        generate_value_ok!(
            r#"Map<Set, List>(Set<U8>(1u8), List<U8>(2u8))"#,
            Value::Map {
                key_type_id: TYPE_SET,
                value_type_id: TYPE_LIST,
                elements: vec![
                    Value::Set {
                        element_type_id: TYPE_U8,
                        elements: vec![Value::U8 { value: 1 }]
                    },
                    Value::List {
                        element_type_id: TYPE_U8,
                        elements: vec![Value::U8 { value: 2 }]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_failures() {
        generate_value_error!(
            r#"ComponentAddress(100u32)"#,
            GeneratorError::InvalidValue {
                expected_type: vec![ast::Type::String],
                actual: ast::Value::U32(100),
            }
        );
        generate_value_error!(
            r#"PackageAddress("invalid_package_address")"#,
            GeneratorError::InvalidPackageAddress("invalid_package_address".into())
        );
        generate_value_error!(
            r#"Decimal("invalid_decimal")"#,
            GeneratorError::InvalidDecimal("invalid_decimal".into())
        );
        generate_value_error!(
            r#"Map<String, String>("abc")"#,
            GeneratorError::OddNumberOfElements(1)
        );
    }

    #[test]
    fn test_instructions() {
        generate_instruction_ok!(
            r#"TAKE_FROM_WORKTOP_BY_AMOUNT  Decimal("1.0")  ResourceAddress("resource_sim1qr9alp6h38ggejqvjl3fzkujpqj2d84gmqy72zuluzwsykwvak")  Bucket("xrd_bucket");"#,
            Instruction::TakeFromWorktopByAmount {
                amount: Decimal::from(1),
                resource_address: ResourceAddress::from_str(
                    "resource_sim1qr9alp6h38ggejqvjl3fzkujpqj2d84gmqy72zuluzwsykwvak"
                )
                .unwrap(),
            }
        );
        generate_instruction_ok!(
            r#"TAKE_FROM_WORKTOP  ResourceAddress("resource_sim1qr9alp6h38ggejqvjl3fzkujpqj2d84gmqy72zuluzwsykwvak")  Bucket("xrd_bucket");"#,
            Instruction::TakeFromWorktop {
                resource_address: ResourceAddress::from_str(
                    "resource_sim1qr9alp6h38ggejqvjl3fzkujpqj2d84gmqy72zuluzwsykwvak"
                )
                .unwrap(),
            }
        );
        generate_instruction_ok!(
            r#"ASSERT_WORKTOP_CONTAINS_BY_AMOUNT  Decimal("1.0")  ResourceAddress("resource_sim1qr9alp6h38ggejqvjl3fzkujpqj2d84gmqy72zuluzwsykwvak");"#,
            Instruction::AssertWorktopContainsByAmount {
                amount: Decimal::from(1),
                resource_address: ResourceAddress::from_str(
                    "resource_sim1qr9alp6h38ggejqvjl3fzkujpqj2d84gmqy72zuluzwsykwvak"
                )
                .unwrap(),
            }
        );
        generate_instruction_ok!(
            r#"CALL_FUNCTION  PackageAddress("package_sim1q8gl2qqsusgzmz92es68wy2fr7zjc523xj57eanm597qrz3dx7")  "Airdrop"  "new"  500u32  Map<String, U8>("key", 1u8);"#,
            Instruction::CallFunction {
                package_address: PackageAddress::from_str(
                    "package_sim1q8gl2qqsusgzmz92es68wy2fr7zjc523xj57eanm597qrz3dx7".into()
                )
                .unwrap(),
                blueprint_name: "Airdrop".into(),
                method_name: "new".to_string(),
                arg: to_struct!(500u32, HashMap::from([("key", 1u8),]))
            }
        );
        generate_instruction_ok!(
            r#"CALL_METHOD  ComponentAddress("component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum")  "refill";"#,
            Instruction::CallMethod {
                component_address: ComponentAddress::from_str(
                    "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum".into()
                )
                .unwrap(),
                method_name: "refill".to_string(),
                arg: to_struct!()
            }
        );
        generate_instruction_ok!(
            r#"CALL_METHOD  ComponentAddress("component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum")  "refill"  NonFungibleId("a1200000000000c84e676dc11b000000000000000000000000000000000000000000000000")  NonFungibleId("0c070000005363727970746f")  NonFungibleAddress("030000000000000000000000000000000000000000000000000000040c070000005363727970746f")  NonFungibleId("b10400000000020000");"#,
            Instruction::CallMethod {
                component_address: ComponentAddress::from_str(
                    "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum".into()
                )
                .unwrap(),
                method_name: "refill".to_string(),
                arg: to_struct!(
                    NonFungibleId(scrypto_encode(&scrypto::dec!("2"))),
                    NonFungibleId(scrypto_encode(&String::from("Scrypto"))),
                    NonFungibleAddress::new(RADIX_TOKEN, NonFungibleId(scrypto_encode(&String::from("Scrypto")))),
                    // TODO: In the future, we should neither be able to create a `NonFungibleId` of a bucket or pass 
                    // that through the manifest.
                    NonFungibleId(scrypto_encode(&scrypto::resource::Bucket(512)))
                )
            }
        );
        generate_instruction_ok!(
            r#"CALL_METHOD_WITH_ALL_RESOURCES  ComponentAddress("account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064") "deposit_batch";"#,
            Instruction::CallMethodWithAllResources {
                component_address: ComponentAddress::from_str(
                    "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064".into()
                )
                .unwrap(),
                method: "deposit_batch".into(),
            }
        );
    }

    #[test]
    fn test_transaction() {
        let tx = include_str!("../../examples/complex.rtm");
        let code = vec![
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x04, 0x05, 0x01, 0x70, 0x01, 0x01,
            0x01, 0x05, 0x03, 0x01, 0x00, 0x10, 0x06, 0x19, 0x03, 0x7f, 0x01, 0x41, 0x80, 0x80,
            0xc0, 0x00, 0x0b, 0x7f, 0x00, 0x41, 0x80, 0x80, 0xc0, 0x00, 0x0b, 0x7f, 0x00, 0x41,
            0x80, 0x80, 0xc0, 0x00, 0x0b, 0x07, 0x25, 0x03, 0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72,
            0x79, 0x02, 0x00, 0x0a, 0x5f, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x65, 0x6e, 0x64,
            0x03, 0x01, 0x0b, 0x5f, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x5f, 0x62, 0x61, 0x73, 0x65,
            0x03, 0x02, 0x00, 0x19, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x07, 0x12, 0x01, 0x00, 0x0f,
            0x5f, 0x5f, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65,
            0x72, 0x00, 0x4d, 0x09, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x72, 0x73, 0x02,
            0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x01, 0x04, 0x52, 0x75, 0x73,
            0x74, 0x00, 0x0c, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x65, 0x64, 0x2d, 0x62,
            0x79, 0x01, 0x05, 0x72, 0x75, 0x73, 0x74, 0x63, 0x1d, 0x31, 0x2e, 0x35, 0x39, 0x2e,
            0x30, 0x20, 0x28, 0x39, 0x64, 0x31, 0x62, 0x32, 0x31, 0x30, 0x36, 0x65, 0x20, 0x32,
            0x30, 0x32, 0x32, 0x2d, 0x30, 0x32, 0x2d, 0x32, 0x33, 0x29,
        ];
        let package = Package {
            code,
            blueprints: HashMap::new(),
        };
        let encoded_package = scrypto_encode(&package);

        assert_eq!(
            crate::manifest::compile(tx, &Network::LocalSimulator)
                .unwrap()
                .instructions,
            vec![
                Instruction::CallMethod {
                    component_address: ComponentAddress::from_str(
                        "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064".into()
                    )
                    .unwrap(),
                    method_name: "withdraw_by_amount".to_string(),
                    arg: to_struct!(
                        Decimal::from(5u32),
                        ResourceAddress::from_str(
                            "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                        )
                        .unwrap()
                    )
                },
                Instruction::TakeFromWorktopByAmount {
                    amount: Decimal::from(2),
                    resource_address: ResourceAddress::from_str(
                        "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                    )
                    .unwrap(),
                },
                Instruction::CallMethod {
                    component_address: ComponentAddress::from_str(
                        "component_sim1q2f9vmyrmeladvz0ejfttcztqv3genlsgpu9vue83mcs835hum".into()
                    )
                    .unwrap(),
                    method_name: "buy_gumball".to_string(),
                    arg: to_struct!(scrypto::resource::Bucket(512))
                },
                Instruction::AssertWorktopContainsByAmount {
                    amount: Decimal::from(3),
                    resource_address: ResourceAddress::from_str(
                        "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                    )
                    .unwrap(),
                },
                Instruction::AssertWorktopContains {
                    resource_address: ResourceAddress::from_str(
                        "resource_sim1qzhdk7tq68u8msj38r6v6yqa5myc64ejx3ud20zlh9gseqtux6"
                    )
                    .unwrap(),
                },
                Instruction::TakeFromWorktop {
                    resource_address: ResourceAddress::from_str(
                        "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                    )
                    .unwrap(),
                },
                Instruction::CreateProofFromBucket { bucket_id: 513 },
                Instruction::CloneProof { proof_id: 514 },
                Instruction::DropProof { proof_id: 514 },
                Instruction::DropProof { proof_id: 515 },
                Instruction::CallMethod {
                    component_address: ComponentAddress::from_str(
                        "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064".into()
                    )
                    .unwrap(),
                    method_name: "create_proof_by_amount".to_string(),
                    arg: to_struct!(
                        Decimal::from(5u32),
                        ResourceAddress::from_str(
                            "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                        )
                        .unwrap()
                    )
                },
                Instruction::PopFromAuthZone,
                Instruction::DropProof { proof_id: 516 },
                Instruction::ReturnToWorktop { bucket_id: 513 },
                Instruction::TakeFromWorktopByIds {
                    ids: BTreeSet::from([
                        NonFungibleId::from_str("0905000000").unwrap(),
                        NonFungibleId::from_str("0907000000").unwrap(),
                    ]),
                    resource_address: ResourceAddress::from_str(
                        "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag"
                    )
                    .unwrap()
                },
                Instruction::CallMethodWithAllResources {
                    component_address: ComponentAddress::from_str(
                        "account_sim1q02r73u7nv47h80e30pc3q6ylsj7mgvparm3pnsm780qgsy064".into()
                    )
                    .unwrap(),
                    method: "deposit_batch".into(),
                },
                Instruction::DropAllProofs,
                Instruction::PublishPackage {
                    package: encoded_package.clone()
                },
            ]
        );
    }
}
