(function() {var type_impls = {
"radix_engine_queries":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html#tymethod.new\" class=\"fn\">new</a>(amount: <a class=\"struct\" href=\"radix_engine_common/math/decimal/struct.Decimal.html\" title=\"struct radix_engine_common::math::decimal::Decimal\">Decimal</a>) -&gt; <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h4></section></div></details>",0,"radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<section id=\"impl-Eq-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-Eq-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section>","Eq","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Categorize%3CScryptoCustomValueKind%3E-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-Categorize%3CScryptoCustomValueKind%3E-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sbor/categorize/trait.Categorize.html\" title=\"trait sbor::categorize::Categorize\">Categorize</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/categorize/trait.Categorize.html#tymethod.value_kind\" class=\"fn\">value_kind</a>() -&gt; <a class=\"enum\" href=\"sbor/value_kind/enum.ValueKind.html\" title=\"enum sbor::value_kind::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;</h4></section></div></details>","Categorize<ScryptoCustomValueKind>","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-Debug-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<section id=\"impl-StructuralPartialEq-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-StructuralPartialEq-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section>","StructuralPartialEq","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SborTuple%3CScryptoCustomValueKind%3E-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-SborTuple%3CScryptoCustomValueKind%3E-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sbor/categorize/trait.SborTuple.html\" title=\"trait sbor::categorize::SborTuple\">SborTuple</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_length\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.get_length\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/categorize/trait.SborTuple.html#tymethod.get_length\" class=\"fn\">get_length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a></h4></section></div></details>","SborTuple<ScryptoCustomValueKind>","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode%3CScryptoCustomValueKind,+E%3E-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-Encode%3CScryptoCustomValueKind,+E%3E-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E&gt; <a class=\"trait\" href=\"sbor/encode/trait.Encode.html\" title=\"trait sbor::encode::Encode\">Encode</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, E&gt; for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a><div class=\"where\">where\n    E: <a class=\"trait\" href=\"sbor/encoder/trait.Encoder.html\" title=\"trait sbor::encoder::Encoder\">Encoder</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.encode_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/encode/trait.Encode.html#tymethod.encode_value_kind\" class=\"fn\">encode_value_kind</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"sbor/encoder/enum.EncodeError.html\" title=\"enum sbor::encoder::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR value’s kind to the encoder</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_body\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.encode_body\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/encode/trait.Encode.html#tymethod.encode_body\" class=\"fn\">encode_body</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"sbor/encoder/enum.EncodeError.html\" title=\"enum sbor::encoder::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR body of the type to the encoder. <a href=\"sbor/encode/trait.Encode.html#tymethod.encode_body\">Read more</a></div></details></div></details>","Encode<ScryptoCustomValueKind, E>","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ScryptoEvent-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-ScryptoEvent-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_interface/traits/event/trait.ScryptoEvent.html\" title=\"trait radix_engine_interface::traits::event::ScryptoEvent\">ScryptoEvent</a> for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.EVENT_NAME\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#associatedconstant.EVENT_NAME\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"radix_engine_interface/traits/event/trait.ScryptoEvent.html#associatedconstant.EVENT_NAME\" class=\"constant\">EVENT_NAME</a>: &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.str.html\">str</a> = &quot;DepositEvent&quot;</h4></section></div></details>","ScryptoEvent","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Describe%3CScryptoCustomTypeKind%3E-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-Describe%3CScryptoCustomTypeKind%3E-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sbor/schema/describe/trait.Describe.html\" title=\"trait sbor::schema::describe::Describe\">Describe</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_schema/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_schema::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.TYPE_ID\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#associatedconstant.TYPE_ID\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"sbor/schema/describe/trait.Describe.html#associatedconstant.TYPE_ID\" class=\"constant\">TYPE_ID</a>: <a class=\"enum\" href=\"sbor/schema/type_link/enum.RustTypeId.html\" title=\"enum sbor::schema::type_link::RustTypeId\">RustTypeId</a> = _</h4></section></summary><div class='docblock'>The <code>TYPE_ID</code> should give a unique identifier for its SBOR schema type.\nAn SBOR schema type capture details about the SBOR payload, how it should be interpreted, validated and displayed. <a href=\"sbor/schema/describe/trait.Describe.html#associatedconstant.TYPE_ID\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.type_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/schema/describe/trait.Describe.html#tymethod.type_data\" class=\"fn\">type_data</a>() -&gt; <a class=\"struct\" href=\"sbor/schema/type_data/struct.TypeData.html\" title=\"struct sbor::schema::type_data::TypeData\">TypeData</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_schema/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_schema::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>, <a class=\"enum\" href=\"sbor/schema/type_link/enum.RustTypeId.html\" title=\"enum sbor::schema::type_link::RustTypeId\">RustTypeId</a>&gt;</h4></section></summary><div class='docblock'>Returns the local schema for the given type. <a href=\"sbor/schema/describe/trait.Describe.html#tymethod.type_data\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_all_dependencies\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.add_all_dependencies\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/schema/describe/trait.Describe.html#method.add_all_dependencies\" class=\"fn\">add_all_dependencies</a>(aggregator: &amp;mut <a class=\"struct\" href=\"sbor/schema/type_aggregator/struct.TypeAggregator.html\" title=\"struct sbor::schema::type_aggregator::TypeAggregator\">TypeAggregator</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_schema/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_schema::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt;)</h4></section></summary><div class='docblock'>For each type referenced in <code>get_local_type_data</code>, we need to ensure that the type and all of its own references\nget added to the aggregator. <a href=\"sbor/schema/describe/trait.Describe.html#method.add_all_dependencies\">Read more</a></div></details></div></details>","Describe<ScryptoCustomTypeKind>","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode%3CScryptoCustomValueKind,+D%3E-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-Decode%3CScryptoCustomValueKind,+D%3E-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"sbor/decode/trait.Decode.html\" title=\"trait sbor::decode::Decode\">Decode</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, D&gt; for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a><div class=\"where\">where\n    D: <a class=\"trait\" href=\"sbor/decoder/trait.Decoder.html\" title=\"trait sbor::decoder::Decoder\">Decoder</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_body_with_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.decode_body_with_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/decode/trait.Decode.html#tymethod.decode_body_with_value_kind\" class=\"fn\">decode_body_with_value_kind</a>(\n    decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut D</a>,\n    value_kind: <a class=\"enum\" href=\"sbor/value_kind/enum.ValueKind.html\" title=\"enum sbor::value_kind::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a>, <a class=\"enum\" href=\"sbor/decoder/enum.DecodeError.html\" title=\"enum sbor::decoder::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decodes the type from the decoder, which should match a preloaded value kind. <a href=\"sbor/decode/trait.Decode.html#tymethod.decode_body_with_value_kind\">Read more</a></div></details></div></details>","Decode<ScryptoCustomValueKind, D>","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-DepositEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#impl-PartialEq-for-DepositEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/events/fungible_vault.rs.html#19-25\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/fungible_vault/struct.DepositEvent.html\" title=\"struct radix_engine_queries::typed_substate_layout::fungible_vault::DepositEvent\">DepositEvent</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","radix_engine_queries::typed_native_events::FungibleVaultDepositEvent"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()