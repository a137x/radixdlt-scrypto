(function() {var type_impls = {
"scrypto_test":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VersionedPackageBlueprintVersionDefinitionVersion-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/package/substates.rs.html#4-114\">source</a><a href=\"#impl-VersionedPackageBlueprintVersionDefinitionVersion-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/trait.VersionedPackageBlueprintVersionDefinitionVersion.html\" title=\"trait scrypto_test::prelude::VersionedPackageBlueprintVersionDefinitionVersion\">VersionedPackageBlueprintVersionDefinitionVersion</a> for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Versioned\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Versioned\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"scrypto_test/prelude/trait.VersionedPackageBlueprintVersionDefinitionVersion.html#associatedtype.Versioned\" class=\"associatedtype\">Versioned</a> = <a class=\"enum\" href=\"scrypto_test/prelude/enum.VersionedPackageBlueprintVersionDefinition.html\" title=\"enum scrypto_test::prelude::VersionedPackageBlueprintVersionDefinition\">VersionedPackageBlueprintVersionDefinition</a></h4></section><section id=\"method.into_versioned\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/package/substates.rs.html#4-114\">source</a><a href=\"#method.into_versioned\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/trait.VersionedPackageBlueprintVersionDefinitionVersion.html#tymethod.into_versioned\" class=\"fn\">into_versioned</a>(\n    self\n) -&gt; &lt;<a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a> as <a class=\"trait\" href=\"scrypto_test/prelude/trait.VersionedPackageBlueprintVersionDefinitionVersion.html\" title=\"trait scrypto_test::prelude::VersionedPackageBlueprintVersionDefinitionVersion\">VersionedPackageBlueprintVersionDefinitionVersion</a>&gt;::<a class=\"associatedtype\" href=\"scrypto_test/prelude/trait.VersionedPackageBlueprintVersionDefinitionVersion.html#associatedtype.Versioned\" title=\"type scrypto_test::prelude::VersionedPackageBlueprintVersionDefinitionVersion::Versioned\">Versioned</a></h4></section></div></details>","VersionedPackageBlueprintVersionDefinitionVersion","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-KeyValueEntryContentSource%3CPackageBlueprintVersionDefinitionEntryPayload%3E-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/package/substates.rs.html#4-114\">source</a><a href=\"#impl-KeyValueEntryContentSource%3CPackageBlueprintVersionDefinitionEntryPayload%3E-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/trait.KeyValueEntryContentSource.html\" title=\"trait scrypto_test::prelude::KeyValueEntryContentSource\">KeyValueEntryContentSource</a>&lt;<a class=\"struct\" href=\"scrypto_test/prelude/struct.PackageBlueprintVersionDefinitionEntryPayload.html\" title=\"struct scrypto_test::prelude::PackageBlueprintVersionDefinitionEntryPayload\">PackageBlueprintVersionDefinitionEntryPayload</a>&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.into_content\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/package/substates.rs.html#4-114\">source</a><a href=\"#method.into_content\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/trait.KeyValueEntryContentSource.html#tymethod.into_content\" class=\"fn\">into_content</a>(self) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/enum.VersionedPackageBlueprintVersionDefinition.html\" title=\"enum scrypto_test::prelude::VersionedPackageBlueprintVersionDefinition\">VersionedPackageBlueprintVersionDefinition</a></h4></section><section id=\"method.into_payload\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/models/payloads.rs.html#164\">source</a><a href=\"#method.into_payload\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/trait.KeyValueEntryContentSource.html#method.into_payload\" class=\"fn\">into_payload</a>(self) -&gt; Payload</h4></section><section id=\"method.into_locked_substate\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/models/payloads.rs.html#168\">source</a><a href=\"#method.into_locked_substate\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/trait.KeyValueEntryContentSource.html#method.into_locked_substate\" class=\"fn\">into_locked_substate</a>(self) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/enum.KeyValueEntrySubstate.html\" title=\"enum scrypto_test::prelude::KeyValueEntrySubstate\">KeyValueEntrySubstate</a>&lt;Payload&gt;</h4></section><section id=\"method.into_unlocked_substate\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/models/payloads.rs.html#172\">source</a><a href=\"#method.into_unlocked_substate\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/trait.KeyValueEntryContentSource.html#method.into_unlocked_substate\" class=\"fn\">into_unlocked_substate</a>(self) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/enum.KeyValueEntrySubstate.html\" title=\"enum scrypto_test::prelude::KeyValueEntrySubstate\">KeyValueEntrySubstate</a>&lt;Payload&gt;</h4></section></div></details>","KeyValueEntryContentSource<PackageBlueprintVersionDefinitionEntryPayload>","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode%3CScryptoCustomValueKind,+D%3E-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-Decode%3CScryptoCustomValueKind,+D%3E-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Decode.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Decode\">Decode</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, D&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a><div class=\"where\">where\n    D: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Decoder.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::Decoder\">Decoder</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_body_with_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.decode_body_with_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Decode.html#tymethod.decode_body_with_value_kind\" class=\"fn\">decode_body_with_value_kind</a>(\n    decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut D</a>,\n    value_kind: <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.ValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;\n) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.Result.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::Result\">Result</a>&lt;<a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a>, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.DecodeError.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decodes the type from the decoder, which should match a preloaded value kind. <a href=\"scrypto_test/prelude/radix_engine_common/trait.Decode.html#tymethod.decode_body_with_value_kind\">Read more</a></div></details></div></details>","Decode<ScryptoCustomValueKind, D>","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Describe%3CScryptoCustomTypeKind%3E-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-Describe%3CScryptoCustomTypeKind%3E-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::Describe\">Describe</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomTypeKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.TYPE_ID\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#associatedconstant.TYPE_ID\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#associatedconstant.TYPE_ID\" class=\"constant\">TYPE_ID</a>: <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.RustTypeId.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::RustTypeId\">RustTypeId</a> = _</h4></section></summary><div class='docblock'>The <code>TYPE_ID</code> should give a unique identifier for its SBOR schema type.\nAn SBOR schema type capture details about the SBOR payload, how it should be interpreted, validated and displayed. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#associatedconstant.TYPE_ID\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.type_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#tymethod.type_data\" class=\"fn\">type_data</a>() -&gt; <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/struct.TypeData.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::prelude::TypeData\">TypeData</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomTypeKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.RustTypeId.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::RustTypeId\">RustTypeId</a>&gt;</h4></section></summary><div class='docblock'>Returns the local schema for the given type. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#tymethod.type_data\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_all_dependencies\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.add_all_dependencies\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#method.add_all_dependencies\" class=\"fn\">add_all_dependencies</a>(aggregator: &amp;mut <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/struct.TypeAggregator.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::prelude::TypeAggregator\">TypeAggregator</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomTypeKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt;)</h4></section></summary><div class='docblock'>For each type referenced in <code>get_local_type_data</code>, we need to ensure that the type and all of its own references\nget added to the aggregator. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#method.add_all_dependencies\">Read more</a></div></details></div></details>","Describe<ScryptoCustomTypeKind>","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode%3CScryptoCustomValueKind,+E%3E-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-Encode%3CScryptoCustomValueKind,+E%3E-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E&gt; <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Encode\">Encode</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, E&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a><div class=\"where\">where\n    E: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Encoder.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::Encoder\">Encoder</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.encode_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html#tymethod.encode_value_kind\" class=\"fn\">encode_value_kind</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.Result.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.EncodeError.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR value’s kind to the encoder</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_body\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.encode_body\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html#tymethod.encode_body\" class=\"fn\">encode_body</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.Result.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.EncodeError.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR body of the type to the encoder. <a href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html#tymethod.encode_body\">Read more</a></div></details></div></details>","Encode<ScryptoCustomValueKind, E>","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<section id=\"impl-Eq-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-Eq-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Eq.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::Eq\">Eq</a> for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section>","Eq","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SborTuple%3CScryptoCustomValueKind%3E-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-SborTuple%3CScryptoCustomValueKind%3E-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.SborTuple.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::SborTuple\">SborTuple</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_length\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.get_length\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.SborTuple.html#tymethod.get_length\" class=\"fn\">get_length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a></h4></section></div></details>","SborTuple<ScryptoCustomValueKind>","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-Debug-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Debug.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::Debug\">Debug</a> for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/fmt/struct.Formatter.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.Result.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/fmt/struct.Error.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<section id=\"impl-StructuralPartialEq-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-StructuralPartialEq-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/rust/marker/trait.StructuralPartialEq.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::rust::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section>","StructuralPartialEq","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-PartialEq-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.PartialEq.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Categorize%3CScryptoCustomValueKind%3E-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-Categorize%3CScryptoCustomValueKind%3E-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Categorize.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Categorize\">Categorize</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Categorize.html#tymethod.value_kind\" class=\"fn\">value_kind</a>() -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.ValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/enum.ScryptoCustomValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;</h4></section></div></details>","Categorize<ScryptoCustomValueKind>","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-BlueprintDefinition\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#impl-Clone-for-BlueprintDefinition\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Clone.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::Clone\">Clone</a> for <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/package/substates.rs.html#117\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"scrypto_test/prelude/struct.BlueprintDefinition.html\" title=\"struct scrypto_test::prelude::BlueprintDefinition\">BlueprintDefinition</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","scrypto_test::prelude::PackageBlueprintVersionDefinitionV1","scrypto_test::prelude::PackageBlueprintVersionDefinition"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()