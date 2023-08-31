(function() {var type_impls = {
"scrypto_test":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode%3CX,+D%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#387-388\">source</a><a href=\"#impl-Decode%3CX,+D%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X, D, Y&gt; <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Decode.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Decode\">Decode</a>&lt;X, D&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/struct.EnumVariantValue.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::EnumVariantValue\">EnumVariantValue</a>&lt;X, Y&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.CustomValueKind.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::CustomValueKind\">CustomValueKind</a>,\n    D: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Decoder.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::Decoder\">Decoder</a>&lt;X&gt;,\n    Y: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Decode.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Decode\">Decode</a>&lt;X, D&gt; + <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.CustomValue.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::CustomValue\">CustomValue</a>&lt;X&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_body_with_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#391-394\">source</a><a href=\"#method.decode_body_with_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Decode.html#tymethod.decode_body_with_value_kind\" class=\"fn\">decode_body_with_value_kind</a>(\n    decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut D</a>,\n    value_kind: <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.ValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::ValueKind\">ValueKind</a>&lt;X&gt;\n) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.Result.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::Result\">Result</a>&lt;<a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/struct.EnumVariantValue.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::EnumVariantValue\">EnumVariantValue</a>&lt;X, Y&gt;, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.DecodeError.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decodes the type from the decoder, which should match a preloaded value kind. <a href=\"scrypto_test/prelude/radix_engine_common/trait.Decode.html#tymethod.decode_body_with_value_kind\">Read more</a></div></details></div></details>","Decode<X, D>","scrypto_test::prelude::radix_engine_common::prelude::ManifestEnumVariantValue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Describe%3CC%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#409-410\">source</a><a href=\"#impl-Describe%3CC%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X, Y, C&gt; <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::Describe\">Describe</a>&lt;C&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/struct.EnumVariantValue.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::EnumVariantValue\">EnumVariantValue</a>&lt;X, Y&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.CustomValueKind.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::CustomValueKind\">CustomValueKind</a>,\n    Y: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.CustomValue.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::CustomValue\">CustomValue</a>&lt;X&gt;,\n    C: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.CustomTypeKind.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::CustomTypeKind\">CustomTypeKind</a>&lt;<a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.RustTypeId.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::RustTypeId\">RustTypeId</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.TYPE_ID\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#412\">source</a><a href=\"#associatedconstant.TYPE_ID\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#associatedconstant.TYPE_ID\" class=\"constant\">TYPE_ID</a>: <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.RustTypeId.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::RustTypeId\">RustTypeId</a> = _</h4></section></summary><div class='docblock'>The <code>TYPE_ID</code> should give a unique identifier for its SBOR schema type.\nAn SBOR schema type capture details about the SBOR payload, how it should be interpreted, validated and displayed. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#associatedconstant.TYPE_ID\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#414\">source</a><a href=\"#method.type_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#tymethod.type_data\" class=\"fn\">type_data</a>() -&gt; <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/struct.TypeData.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::prelude::TypeData\">TypeData</a>&lt;C, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.RustTypeId.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::RustTypeId\">RustTypeId</a>&gt;</h4></section></summary><div class='docblock'>Returns the local schema for the given type. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#tymethod.type_data\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_all_dependencies\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/schema/describe.rs.html#59\">source</a><a href=\"#method.add_all_dependencies\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#method.add_all_dependencies\" class=\"fn\">add_all_dependencies</a>(aggregator: &amp;mut <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/struct.TypeAggregator.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::prelude::TypeAggregator\">TypeAggregator</a>&lt;C&gt;)</h4></section></summary><div class='docblock'>For each type referenced in <code>get_local_type_data</code>, we need to ensure that the type and all of its own references\nget added to the aggregator. <a href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Describe.html#method.add_all_dependencies\">Read more</a></div></details></div></details>","Describe<C>","scrypto_test::prelude::radix_engine_common::prelude::ManifestEnumVariantValue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode%3CX,+E%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#370-371\">source</a><a href=\"#impl-Encode%3CX,+E%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X, E, Y&gt; <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Encode\">Encode</a>&lt;X, E&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/struct.EnumVariantValue.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::EnumVariantValue\">EnumVariantValue</a>&lt;X, Y&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.CustomValueKind.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::CustomValueKind\">CustomValueKind</a>,\n    E: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.Encoder.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::Encoder\">Encoder</a>&lt;X&gt;,\n    Y: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Encode\">Encode</a>&lt;X, E&gt; + <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.CustomValue.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::CustomValue\">CustomValue</a>&lt;X&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#373\">source</a><a href=\"#method.encode_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html#tymethod.encode_value_kind\" class=\"fn\">encode_value_kind</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.Result.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.EncodeError.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR value’s kind to the encoder</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_body\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#377\">source</a><a href=\"#method.encode_body\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html#tymethod.encode_body\" class=\"fn\">encode_body</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.Result.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.EncodeError.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR body of the type to the encoder. <a href=\"scrypto_test/prelude/radix_engine_common/trait.Encode.html#tymethod.encode_body\">Read more</a></div></details></div></details>","Encode<X, E>","scrypto_test::prelude::radix_engine_common::prelude::ManifestEnumVariantValue"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Categorize%3CX%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#364\">source</a><a href=\"#impl-Categorize%3CX%3E-for-EnumVariantValue%3CX,+Y%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X, Y&gt; <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/trait.Categorize.html\" title=\"trait scrypto_test::prelude::radix_engine_common::Categorize\">Categorize</a>&lt;X&gt; for <a class=\"struct\" href=\"scrypto_test/prelude/radix_engine_common/prelude/struct.EnumVariantValue.html\" title=\"struct scrypto_test::prelude::radix_engine_common::prelude::EnumVariantValue\">EnumVariantValue</a>&lt;X, Y&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/trait.CustomValueKind.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::prelude::CustomValueKind\">CustomValueKind</a>,\n    Y: <a class=\"trait\" href=\"scrypto_test/prelude/radix_engine_common/prelude/trait.CustomValue.html\" title=\"trait scrypto_test::prelude::radix_engine_common::prelude::CustomValue\">CustomValue</a>&lt;X&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value.rs.html#365\">source</a><a href=\"#method.value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"scrypto_test/prelude/radix_engine_common/trait.Categorize.html#tymethod.value_kind\" class=\"fn\">value_kind</a>() -&gt; <a class=\"enum\" href=\"scrypto_test/prelude/radix_engine_common/prelude/prelude/enum.ValueKind.html\" title=\"enum scrypto_test::prelude::radix_engine_common::prelude::prelude::ValueKind\">ValueKind</a>&lt;X&gt;</h4></section></div></details>","Categorize<X>","scrypto_test::prelude::radix_engine_common::prelude::ManifestEnumVariantValue"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()