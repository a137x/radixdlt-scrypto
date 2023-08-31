(function() {var type_impls = {
"radix_engine_interface":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-Hash-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/rust/hash/trait.Hash.html\" title=\"trait radix_engine_interface::prelude::rust::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/rust/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H: <a class=\"trait\" href=\"radix_engine_interface/prelude/rust/hash/trait.Hasher.html\" title=\"trait radix_engine_interface::prelude::rust::hash::Hasher\">Hasher</a>&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut __H</a>)</h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"radix_engine_interface/prelude/rust/hash/trait.Hasher.html\" title=\"trait radix_engine_interface::prelude::rust::hash::Hasher\"><code>Hasher</code></a>. <a href=\"radix_engine_interface/prelude/rust/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/hash/mod.rs.html#238-240\">source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/rust/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.slice.html\">[Self]</a>, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"radix_engine_interface/prelude/rust/hash/trait.Hasher.html\" title=\"trait radix_engine_interface::prelude::rust::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.Sized.html\" title=\"trait radix_engine_interface::prelude::prelude::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"radix_engine_interface/prelude/rust/hash/trait.Hasher.html\" title=\"trait radix_engine_interface::prelude::rust::hash::Hasher\"><code>Hasher</code></a>. <a href=\"radix_engine_interface/prelude/rust/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SborTuple%3CScryptoCustomValueKind%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-SborTuple%3CScryptoCustomValueKind%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.SborTuple.html\" title=\"trait radix_engine_interface::prelude::prelude::SborTuple\">SborTuple</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a><div class=\"where\">where\n    for&lt;'b_&gt; &amp;'b_ <a class=\"struct\" href=\"radix_engine_interface/data/scrypto/prelude/struct.Own.html\" title=\"struct radix_engine_interface::data::scrypto::prelude::Own\">Own</a>: <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.SborTuple.html\" title=\"trait radix_engine_interface::prelude::prelude::SborTuple\">SborTuple</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_length\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.get_length\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.SborTuple.html#tymethod.get_length\" class=\"fn\">get_length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a></h4></section></div></details>","SborTuple<ScryptoCustomValueKind>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode%3CScryptoCustomValueKind,+E%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-Encode%3CScryptoCustomValueKind,+E%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E: <a class=\"trait\" href=\"radix_engine_interface/prelude/trait.Encoder.html\" title=\"trait radix_engine_interface::prelude::Encoder\">Encoder</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;&gt; <a class=\"trait\" href=\"radix_engine_interface/trait.Encode.html\" title=\"trait radix_engine_interface::Encode\">Encode</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, E&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.encode_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/trait.Encode.html#tymethod.encode_value_kind\" class=\"fn\">encode_value_kind</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.Result.html\" title=\"enum radix_engine_interface::prelude::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.EncodeError.html\" title=\"enum radix_engine_interface::prelude::prelude::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR value’s kind to the encoder</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_body\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.encode_body\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/trait.Encode.html#tymethod.encode_body\" class=\"fn\">encode_body</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.Result.html\" title=\"enum radix_engine_interface::prelude::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.EncodeError.html\" title=\"enum radix_engine_interface::prelude::prelude::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR body of the type to the encoder. <a href=\"radix_engine_interface/trait.Encode.html#tymethod.encode_body\">Read more</a></div></details></div></details>","Encode<ScryptoCustomValueKind, E>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<section id=\"impl-Eq-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-Eq-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.Eq.html\" title=\"trait radix_engine_interface::prelude::prelude::Eq\">Eq</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section>","Eq","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SborEnum%3CScryptoCustomValueKind%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-SborEnum%3CScryptoCustomValueKind%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.SborEnum.html\" title=\"trait radix_engine_interface::prelude::prelude::SborEnum\">SborEnum</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a><div class=\"where\">where\n    for&lt;'b_&gt; &amp;'b_ <a class=\"struct\" href=\"radix_engine_interface/data/scrypto/prelude/struct.Own.html\" title=\"struct radix_engine_interface::data::scrypto::prelude::Own\">Own</a>: <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.SborEnum.html\" title=\"trait radix_engine_interface::prelude::prelude::SborEnum\">SborEnum</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_discriminator\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.get_discriminator\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.SborEnum.html#tymethod.get_discriminator\" class=\"fn\">get_discriminator</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.get_length\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.get_length\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.SborEnum.html#tymethod.get_length\" class=\"fn\">get_length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a></h4></section></div></details>","SborEnum<ScryptoCustomValueKind>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CFungibleVault%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#114-118\">source</a><a href=\"#impl-From%3CFungibleVault%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.From.html\" title=\"trait radix_engine_interface::prelude::prelude::From\">From</a>&lt;<a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.FungibleVault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::FungibleVault\">FungibleVault</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#115-117\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.From.html#tymethod.from\" class=\"fn\">from</a>(value: <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.FungibleVault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::FungibleVault\">FungibleVault</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<FungibleVault>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CNonFungibleVault%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#120-124\">source</a><a href=\"#impl-From%3CNonFungibleVault%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.From.html\" title=\"trait radix_engine_interface::prelude::prelude::From\">From</a>&lt;<a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.NonFungibleVault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::NonFungibleVault\">NonFungibleVault</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#121-123\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.From.html#tymethod.from\" class=\"fn\">from</a>(value: <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.NonFungibleVault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::NonFungibleVault\">NonFungibleVault</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<NonFungibleVault>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Describe%3CScryptoCustomTypeKind%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#130-137\">source</a><a href=\"#impl-Describe%3CScryptoCustomTypeKind%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/trait.Describe.html\" title=\"trait radix_engine_interface::prelude::Describe\">Describe</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.TYPE_ID\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#131-132\">source</a><a href=\"#associatedconstant.TYPE_ID\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"radix_engine_interface/prelude/trait.Describe.html#associatedconstant.TYPE_ID\" class=\"constant\">TYPE_ID</a>: <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.RustTypeId.html\" title=\"enum radix_engine_interface::prelude::prelude::RustTypeId\">RustTypeId</a> = _</h4></section></summary><div class='docblock'>The <code>TYPE_ID</code> should give a unique identifier for its SBOR schema type.\nAn SBOR schema type capture details about the SBOR payload, how it should be interpreted, validated and displayed. <a href=\"radix_engine_interface/prelude/trait.Describe.html#associatedconstant.TYPE_ID\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#134-136\">source</a><a href=\"#method.type_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/trait.Describe.html#tymethod.type_data\" class=\"fn\">type_data</a>() -&gt; <a class=\"struct\" href=\"radix_engine_interface/prelude/prelude/struct.TypeData.html\" title=\"struct radix_engine_interface::prelude::prelude::TypeData\">TypeData</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>, <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.RustTypeId.html\" title=\"enum radix_engine_interface::prelude::prelude::RustTypeId\">RustTypeId</a>&gt;</h4></section></summary><div class='docblock'>Returns the local schema for the given type. <a href=\"radix_engine_interface/prelude/trait.Describe.html#tymethod.type_data\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_all_dependencies\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/schema/describe.rs.html#59\">source</a><a href=\"#method.add_all_dependencies\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/trait.Describe.html#method.add_all_dependencies\" class=\"fn\">add_all_dependencies</a>(aggregator: &amp;mut <a class=\"struct\" href=\"radix_engine_interface/prelude/prelude/struct.TypeAggregator.html\" title=\"struct radix_engine_interface::prelude::prelude::TypeAggregator\">TypeAggregator</a>&lt;C&gt;)</h4></section></summary><div class='docblock'>For each type referenced in <code>get_local_type_data</code>, we need to ensure that the type and all of its own references\nget added to the aggregator. <a href=\"radix_engine_interface/prelude/trait.Describe.html#method.add_all_dependencies\">Read more</a></div></details></div></details>","Describe<ScryptoCustomTypeKind>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Categorize%3CScryptoCustomValueKind%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-Categorize%3CScryptoCustomValueKind%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/trait.Categorize.html\" title=\"trait radix_engine_interface::Categorize\">Categorize</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/trait.Categorize.html#tymethod.value_kind\" class=\"fn\">value_kind</a>() -&gt; <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.ValueKind.html\" title=\"enum radix_engine_interface::prelude::prelude::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;</h4></section></div></details>","Categorize<ScryptoCustomValueKind>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode%3CScryptoCustomValueKind,+D%3E-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-Decode%3CScryptoCustomValueKind,+D%3E-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D: <a class=\"trait\" href=\"radix_engine_interface/prelude/trait.Decoder.html\" title=\"trait radix_engine_interface::prelude::Decoder\">Decoder</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;&gt; <a class=\"trait\" href=\"radix_engine_interface/trait.Decode.html\" title=\"trait radix_engine_interface::Decode\">Decode</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, D&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_body_with_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.decode_body_with_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/trait.Decode.html#tymethod.decode_body_with_value_kind\" class=\"fn\">decode_body_with_value_kind</a>(\n    decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut D</a>,\n    value_kind: <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.ValueKind.html\" title=\"enum radix_engine_interface::prelude::prelude::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine_interface/data/scrypto/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_interface::data::scrypto::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;\n) -&gt; <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.Result.html\" title=\"enum radix_engine_interface::prelude::prelude::Result\">Result</a>&lt;Self, <a class=\"enum\" href=\"radix_engine_interface/prelude/prelude/enum.DecodeError.html\" title=\"enum radix_engine_interface::prelude::prelude::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decodes the type from the decoder, which should match a preloaded value kind. <a href=\"radix_engine_interface/trait.Decode.html#tymethod.decode_body_with_value_kind\">Read more</a></div></details></div></details>","Decode<ScryptoCustomValueKind, D>","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-Debug-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.Debug.html\" title=\"trait radix_engine_interface::prelude::prelude::Debug\">Debug</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"radix_engine_interface/prelude/fmt/struct.Formatter.html\" title=\"struct radix_engine_interface::prelude::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"radix_engine_interface/prelude/fmt/type.Result.html\" title=\"type radix_engine_interface::prelude::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"radix_engine_interface/prelude/prelude/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<section id=\"impl-StructuralPartialEq-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-StructuralPartialEq-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/rust/marker/trait.StructuralPartialEq.html\" title=\"trait radix_engine_interface::prelude::rust::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section>","StructuralPartialEq","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Vault\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#impl-PartialEq-for-Vault\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/prelude/prelude/trait.PartialEq.html\" title=\"trait radix_engine_interface::prelude::prelude::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/vault.rs.html#99\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/vault/struct.Vault.html\" title=\"struct radix_engine_interface::blueprints::resource::vault::Vault\">Vault</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_interface/prelude/prelude/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","radix_engine_interface::blueprints::resource::resource_manager::ResourceManagerCreateEmptyVaultOutput"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()