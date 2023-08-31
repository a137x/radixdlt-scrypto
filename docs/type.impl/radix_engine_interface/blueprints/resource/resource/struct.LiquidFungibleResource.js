(function() {var type_impls = {
"radix_engine_queries":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#36\">source</a><a href=\"#impl-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#37\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html#tymethod.new\" class=\"fn\">new</a>(amount: <a class=\"struct\" href=\"radix_engine_common/math/decimal/struct.Decimal.html\" title=\"struct radix_engine_common::math::decimal::Decimal\">Decimal</a>) -&gt; <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h4></section><section id=\"method.default\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#41\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h4></section><section id=\"method.amount\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#45\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html#tymethod.amount\" class=\"fn\">amount</a>(&amp;self) -&gt; <a class=\"struct\" href=\"radix_engine_common/math/decimal/struct.Decimal.html\" title=\"struct radix_engine_common::math::decimal::Decimal\">Decimal</a></h4></section><section id=\"method.is_empty\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#49\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html#tymethod.is_empty\" class=\"fn\">is_empty</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section><section id=\"method.put\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#53\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html#tymethod.put\" class=\"fn\">put</a>(&amp;mut self, other: <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a>)</h4></section><section id=\"method.take_by_amount\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#60-63\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html#tymethod.take_by_amount\" class=\"fn\">take_by_amount</a>(\n    &amp;mut self,\n    amount_to_take: <a class=\"struct\" href=\"radix_engine_common/math/decimal/struct.Decimal.html\" title=\"struct radix_engine_common::math::decimal::Decimal\">Decimal</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a>, <a class=\"enum\" href=\"radix_engine_interface/blueprints/resource/resource/enum.ResourceError.html\" title=\"enum radix_engine_interface::blueprints::resource::resource::ResourceError\">ResourceError</a>&gt;</h4></section><section id=\"method.take_all\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#78\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html#tymethod.take_all\" class=\"fn\">take_all</a>(&amp;mut self) -&gt; <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h4></section></div></details>",0,"radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode%3CScryptoCustomValueKind,+D%3E-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-Decode%3CScryptoCustomValueKind,+D%3E-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"sbor/decode/trait.Decode.html\" title=\"trait sbor::decode::Decode\">Decode</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, D&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a><div class=\"where\">where\n    D: <a class=\"trait\" href=\"sbor/decoder/trait.Decoder.html\" title=\"trait sbor::decoder::Decoder\">Decoder</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_body_with_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.decode_body_with_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/decode/trait.Decode.html#tymethod.decode_body_with_value_kind\" class=\"fn\">decode_body_with_value_kind</a>(\n    decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut D</a>,\n    value_kind: <a class=\"enum\" href=\"sbor/value_kind/enum.ValueKind.html\" title=\"enum sbor::value_kind::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a>, <a class=\"enum\" href=\"sbor/decoder/enum.DecodeError.html\" title=\"enum sbor::decoder::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decodes the type from the decoder, which should match a preloaded value kind. <a href=\"sbor/decode/trait.Decode.html#tymethod.decode_body_with_value_kind\">Read more</a></div></details></div></details>","Decode<ScryptoCustomValueKind, D>","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Describe%3CScryptoCustomTypeKind%3E-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-Describe%3CScryptoCustomTypeKind%3E-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sbor/schema/describe/trait.Describe.html\" title=\"trait sbor::schema::describe::Describe\">Describe</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_schema/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_schema::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.TYPE_ID\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#associatedconstant.TYPE_ID\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"sbor/schema/describe/trait.Describe.html#associatedconstant.TYPE_ID\" class=\"constant\">TYPE_ID</a>: <a class=\"enum\" href=\"sbor/schema/type_link/enum.RustTypeId.html\" title=\"enum sbor::schema::type_link::RustTypeId\">RustTypeId</a> = _</h4></section></summary><div class='docblock'>The <code>TYPE_ID</code> should give a unique identifier for its SBOR schema type.\nAn SBOR schema type capture details about the SBOR payload, how it should be interpreted, validated and displayed. <a href=\"sbor/schema/describe/trait.Describe.html#associatedconstant.TYPE_ID\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.type_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/schema/describe/trait.Describe.html#tymethod.type_data\" class=\"fn\">type_data</a>() -&gt; <a class=\"struct\" href=\"sbor/schema/type_data/struct.TypeData.html\" title=\"struct sbor::schema::type_data::TypeData\">TypeData</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_schema/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_schema::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>, <a class=\"enum\" href=\"sbor/schema/type_link/enum.RustTypeId.html\" title=\"enum sbor::schema::type_link::RustTypeId\">RustTypeId</a>&gt;</h4></section></summary><div class='docblock'>Returns the local schema for the given type. <a href=\"sbor/schema/describe/trait.Describe.html#tymethod.type_data\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_all_dependencies\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.add_all_dependencies\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/schema/describe/trait.Describe.html#method.add_all_dependencies\" class=\"fn\">add_all_dependencies</a>(aggregator: &amp;mut <a class=\"struct\" href=\"sbor/schema/type_aggregator/struct.TypeAggregator.html\" title=\"struct sbor::schema::type_aggregator::TypeAggregator\">TypeAggregator</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_schema/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_schema::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt;)</h4></section></summary><div class='docblock'>For each type referenced in <code>get_local_type_data</code>, we need to ensure that the type and all of its own references\nget added to the aggregator. <a href=\"sbor/schema/describe/trait.Describe.html#method.add_all_dependencies\">Read more</a></div></details></div></details>","Describe<ScryptoCustomTypeKind>","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode%3CScryptoCustomValueKind,+E%3E-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-Encode%3CScryptoCustomValueKind,+E%3E-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E&gt; <a class=\"trait\" href=\"sbor/encode/trait.Encode.html\" title=\"trait sbor::encode::Encode\">Encode</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, E&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a><div class=\"where\">where\n    E: <a class=\"trait\" href=\"sbor/encoder/trait.Encoder.html\" title=\"trait sbor::encoder::Encoder\">Encoder</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.encode_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/encode/trait.Encode.html#tymethod.encode_value_kind\" class=\"fn\">encode_value_kind</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"sbor/encoder/enum.EncodeError.html\" title=\"enum sbor::encoder::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR value’s kind to the encoder</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_body\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.encode_body\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/encode/trait.Encode.html#tymethod.encode_body\" class=\"fn\">encode_body</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"sbor/encoder/enum.EncodeError.html\" title=\"enum sbor::encoder::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR body of the type to the encoder. <a href=\"sbor/encode/trait.Encode.html#tymethod.encode_body\">Read more</a></div></details></div></details>","Encode<ScryptoCustomValueKind, E>","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<section id=\"impl-Eq-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-Eq-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section>","Eq","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SborTuple%3CScryptoCustomValueKind%3E-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-SborTuple%3CScryptoCustomValueKind%3E-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sbor/categorize/trait.SborTuple.html\" title=\"trait sbor::categorize::SborTuple\">SborTuple</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a><div class=\"where\">where\n    &amp;'b_ <a class=\"struct\" href=\"radix_engine_common/math/decimal/struct.Decimal.html\" title=\"struct radix_engine_common::math::decimal::Decimal\">Decimal</a>: for&lt;'b_&gt; <a class=\"trait\" href=\"sbor/categorize/trait.SborTuple.html\" title=\"trait sbor::categorize::SborTuple\">SborTuple</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_length\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.get_length\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/categorize/trait.SborTuple.html#tymethod.get_length\" class=\"fn\">get_length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a></h4></section></div></details>","SborTuple<ScryptoCustomValueKind>","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-Debug-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<section id=\"impl-StructuralPartialEq-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-StructuralPartialEq-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section>","StructuralPartialEq","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-PartialEq-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SborEnum%3CScryptoCustomValueKind%3E-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-SborEnum%3CScryptoCustomValueKind%3E-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sbor/categorize/trait.SborEnum.html\" title=\"trait sbor::categorize::SborEnum\">SborEnum</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a><div class=\"where\">where\n    &amp;'b_ <a class=\"struct\" href=\"radix_engine_common/math/decimal/struct.Decimal.html\" title=\"struct radix_engine_common::math::decimal::Decimal\">Decimal</a>: for&lt;'b_&gt; <a class=\"trait\" href=\"sbor/categorize/trait.SborEnum.html\" title=\"trait sbor::categorize::SborEnum\">SborEnum</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_discriminator\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.get_discriminator\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/categorize/trait.SborEnum.html#tymethod.get_discriminator\" class=\"fn\">get_discriminator</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.get_length\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.get_length\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/categorize/trait.SborEnum.html#tymethod.get_length\" class=\"fn\">get_length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a></h4></section></div></details>","SborEnum<ScryptoCustomValueKind>","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Categorize%3CScryptoCustomValueKind%3E-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-Categorize%3CScryptoCustomValueKind%3E-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sbor/categorize/trait.Categorize.html\" title=\"trait sbor::categorize::Categorize\">Categorize</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/categorize/trait.Categorize.html#tymethod.value_kind\" class=\"fn\">value_kind</a>() -&gt; <a class=\"enum\" href=\"sbor/value_kind/enum.ValueKind.html\" title=\"enum sbor::value_kind::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine_common/data/scrypto/custom_value_kind/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine_common::data::scrypto::custom_value_kind::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;</h4></section></div></details>","Categorize<ScryptoCustomValueKind>","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#impl-Clone-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_interface/blueprints/resource/resource.rs.html#15\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.1/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FieldContentSource%3CFungibleVaultBalanceFieldPayload%3E-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/fungible/fungible_vault.rs.html#15-43\">source</a><a href=\"#impl-FieldContentSource%3CFungibleVaultBalanceFieldPayload%3E-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/blueprints/models/payloads/trait.FieldContentSource.html\" title=\"trait radix_engine::blueprints::models::payloads::FieldContentSource\">FieldContentSource</a>&lt;<a class=\"struct\" href=\"radix_engine_queries/typed_substate_layout/struct.FungibleVaultBalanceFieldPayload.html\" title=\"struct radix_engine_queries::typed_substate_layout::FungibleVaultBalanceFieldPayload\">FungibleVaultBalanceFieldPayload</a>&gt; for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.into_content\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/fungible/fungible_vault.rs.html#15-43\">source</a><a href=\"#method.into_content\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/blueprints/models/payloads/trait.FieldContentSource.html#tymethod.into_content\" class=\"fn\">into_content</a>(self) -&gt; <a class=\"enum\" href=\"radix_engine_queries/typed_substate_layout/enum.VersionedFungibleVaultBalance.html\" title=\"enum radix_engine_queries::typed_substate_layout::VersionedFungibleVaultBalance\">VersionedFungibleVaultBalance</a></h4></section><section id=\"method.into_payload\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/models/payloads.rs.html#114\">source</a><a href=\"#method.into_payload\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/blueprints/models/payloads/trait.FieldContentSource.html#method.into_payload\" class=\"fn\">into_payload</a>(self) -&gt; Payload</h4></section><section id=\"method.into_locked_substate\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/models/payloads.rs.html#118\">source</a><a href=\"#method.into_locked_substate\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/blueprints/models/payloads/trait.FieldContentSource.html#method.into_locked_substate\" class=\"fn\">into_locked_substate</a>(self) -&gt; <a class=\"enum\" href=\"radix_engine/system/system_substates/enum.FieldSubstate.html\" title=\"enum radix_engine::system::system_substates::FieldSubstate\">FieldSubstate</a>&lt;Payload&gt;</h4></section><section id=\"method.into_unlocked_substate\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/models/payloads.rs.html#122\">source</a><a href=\"#method.into_unlocked_substate\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/blueprints/models/payloads/trait.FieldContentSource.html#method.into_unlocked_substate\" class=\"fn\">into_unlocked_substate</a>(self) -&gt; <a class=\"enum\" href=\"radix_engine/system/system_substates/enum.FieldSubstate.html\" title=\"enum radix_engine::system::system_substates::FieldSubstate\">FieldSubstate</a>&lt;Payload&gt;</h4></section></div></details>","FieldContentSource<FungibleVaultBalanceFieldPayload>","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VersionedFungibleVaultBalanceVersion-for-LiquidFungibleResource\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/fungible/fungible_vault.rs.html#15-43\">source</a><a href=\"#impl-VersionedFungibleVaultBalanceVersion-for-LiquidFungibleResource\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine_queries/typed_substate_layout/trait.VersionedFungibleVaultBalanceVersion.html\" title=\"trait radix_engine_queries::typed_substate_layout::VersionedFungibleVaultBalanceVersion\">VersionedFungibleVaultBalanceVersion</a> for <a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Versioned\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Versioned\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"radix_engine_queries/typed_substate_layout/trait.VersionedFungibleVaultBalanceVersion.html#associatedtype.Versioned\" class=\"associatedtype\">Versioned</a> = <a class=\"enum\" href=\"radix_engine_queries/typed_substate_layout/enum.VersionedFungibleVaultBalance.html\" title=\"enum radix_engine_queries::typed_substate_layout::VersionedFungibleVaultBalance\">VersionedFungibleVaultBalance</a></h4></section><section id=\"method.into_versioned\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/resource/fungible/fungible_vault.rs.html#15-43\">source</a><a href=\"#method.into_versioned\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine_queries/typed_substate_layout/trait.VersionedFungibleVaultBalanceVersion.html#tymethod.into_versioned\" class=\"fn\">into_versioned</a>(\n    self\n) -&gt; &lt;<a class=\"struct\" href=\"radix_engine_interface/blueprints/resource/resource/struct.LiquidFungibleResource.html\" title=\"struct radix_engine_interface::blueprints::resource::resource::LiquidFungibleResource\">LiquidFungibleResource</a> as <a class=\"trait\" href=\"radix_engine_queries/typed_substate_layout/trait.VersionedFungibleVaultBalanceVersion.html\" title=\"trait radix_engine_queries::typed_substate_layout::VersionedFungibleVaultBalanceVersion\">VersionedFungibleVaultBalanceVersion</a>&gt;::<a class=\"associatedtype\" href=\"radix_engine_queries/typed_substate_layout/trait.VersionedFungibleVaultBalanceVersion.html#associatedtype.Versioned\" title=\"type radix_engine_queries::typed_substate_layout::VersionedFungibleVaultBalanceVersion::Versioned\">Versioned</a></h4></section></div></details>","VersionedFungibleVaultBalanceVersion","radix_engine_queries::typed_substate_layout::FungibleVaultBalance"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()