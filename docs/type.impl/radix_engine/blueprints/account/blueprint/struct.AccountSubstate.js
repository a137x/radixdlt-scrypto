(function() {var type_impls = {
"radix_engine":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-Clone-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.Clone.html\" title=\"trait radix_engine::types::Clone\">Clone</a> for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"radix_engine/types/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"radix_engine/types/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode%3CScryptoCustomValueKind,+E%3E-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-Encode%3CScryptoCustomValueKind,+E%3E-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E: <a class=\"trait\" href=\"radix_engine/types/trait.Encoder.html\" title=\"trait radix_engine::types::Encoder\">Encoder</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;&gt; <a class=\"trait\" href=\"radix_engine/types/trait.Encode.html\" title=\"trait radix_engine::types::Encode\">Encode</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, E&gt; for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.encode_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Encode.html#tymethod.encode_value_kind\" class=\"fn\">encode_value_kind</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Result.html\" title=\"enum radix_engine::types::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"radix_engine/types/enum.EncodeError.html\" title=\"enum radix_engine::types::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR value’s kind to the encoder</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode_body\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.encode_body\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Encode.html#tymethod.encode_body\" class=\"fn\">encode_body</a>(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut E</a>) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Result.html\" title=\"enum radix_engine::types::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"radix_engine/types/enum.EncodeError.html\" title=\"enum radix_engine::types::EncodeError\">EncodeError</a>&gt;</h4></section></summary><div class='docblock'>Encodes the SBOR body of the type to the encoder. <a href=\"radix_engine/types/trait.Encode.html#tymethod.encode_body\">Read more</a></div></details></div></details>","Encode<ScryptoCustomValueKind, E>","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Categorize%3CScryptoCustomValueKind%3E-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-Categorize%3CScryptoCustomValueKind%3E-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.Categorize.html\" title=\"trait radix_engine::types::Categorize\">Categorize</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Categorize.html#tymethod.value_kind\" class=\"fn\">value_kind</a>() -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;</h4></section></div></details>","Categorize<ScryptoCustomValueKind>","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<section id=\"impl-StructuralPartialEq-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-StructuralPartialEq-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/radix_engine_common/prelude/rust/marker/trait.StructuralPartialEq.html\" title=\"trait radix_engine::types::radix_engine_common::prelude::rust::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section>","StructuralPartialEq","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<section id=\"impl-Eq-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-Eq-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.Eq.html\" title=\"trait radix_engine::types::Eq\">Eq</a> for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section>","Eq","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-Debug-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.Debug.html\" title=\"trait radix_engine::types::Debug\">Debug</a> for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"radix_engine/types/radix_engine_common/prelude/fmt/struct.Formatter.html\" title=\"struct radix_engine::types::radix_engine_common::prelude::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"radix_engine/types/radix_engine_common/prelude/fmt/type.Result.html\" title=\"type radix_engine::types::radix_engine_common::prelude::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"radix_engine/types/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SborTuple%3CScryptoCustomValueKind%3E-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-SborTuple%3CScryptoCustomValueKind%3E-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.SborTuple.html\" title=\"trait radix_engine::types::SborTuple\">SborTuple</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_length\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.get_length\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.SborTuple.html#tymethod.get_length\" class=\"fn\">get_length</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a></h4></section></div></details>","SborTuple<ScryptoCustomValueKind>","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode%3CScryptoCustomValueKind,+D%3E-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-Decode%3CScryptoCustomValueKind,+D%3E-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D: <a class=\"trait\" href=\"radix_engine/types/trait.Decoder.html\" title=\"trait radix_engine::types::Decoder\">Decoder</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;&gt; <a class=\"trait\" href=\"radix_engine/types/trait.Decode.html\" title=\"trait radix_engine::types::Decode\">Decode</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>, D&gt; for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_body_with_value_kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.decode_body_with_value_kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Decode.html#tymethod.decode_body_with_value_kind\" class=\"fn\">decode_body_with_value_kind</a>(\n    decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut D</a>,\n    value_kind: <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;\n) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Result.html\" title=\"enum radix_engine::types::Result\">Result</a>&lt;Self, <a class=\"enum\" href=\"radix_engine/types/enum.DecodeError.html\" title=\"enum radix_engine::types::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decodes the type from the decoder, which should match a preloaded value kind. <a href=\"radix_engine/types/trait.Decode.html#tymethod.decode_body_with_value_kind\">Read more</a></div></details></div></details>","Decode<ScryptoCustomValueKind, D>","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Describe%3CScryptoCustomTypeKind%3E-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-Describe%3CScryptoCustomTypeKind%3E-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.Describe.html\" title=\"trait radix_engine::types::Describe\">Describe</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine::types::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt; for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.TYPE_ID\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#associatedconstant.TYPE_ID\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"radix_engine/types/trait.Describe.html#associatedconstant.TYPE_ID\" class=\"constant\">TYPE_ID</a>: <a class=\"enum\" href=\"radix_engine/types/enum.RustTypeId.html\" title=\"enum radix_engine::types::RustTypeId\">RustTypeId</a> = _</h4></section></summary><div class='docblock'>The <code>TYPE_ID</code> should give a unique identifier for its SBOR schema type.\nAn SBOR schema type capture details about the SBOR payload, how it should be interpreted, validated and displayed. <a href=\"radix_engine/types/trait.Describe.html#associatedconstant.TYPE_ID\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.type_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Describe.html#tymethod.type_data\" class=\"fn\">type_data</a>() -&gt; <a class=\"struct\" href=\"radix_engine/types/struct.TypeData.html\" title=\"struct radix_engine::types::TypeData\">TypeData</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine::types::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>, <a class=\"enum\" href=\"radix_engine/types/enum.RustTypeId.html\" title=\"enum radix_engine::types::RustTypeId\">RustTypeId</a>&gt;</h4></section></summary><div class='docblock'>Returns the local schema for the given type. <a href=\"radix_engine/types/trait.Describe.html#tymethod.type_data\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_all_dependencies\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.add_all_dependencies\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Describe.html#method.add_all_dependencies\" class=\"fn\">add_all_dependencies</a>(aggregator: &amp;mut <a class=\"struct\" href=\"radix_engine/types/struct.TypeAggregator.html\" title=\"struct radix_engine::types::TypeAggregator\">TypeAggregator</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomTypeKind.html\" title=\"enum radix_engine::types::ScryptoCustomTypeKind\">ScryptoCustomTypeKind</a>&gt;)</h4></section></summary><div class='docblock'>For each type referenced in <code>get_local_type_data</code>, we need to ensure that the type and all of its own references\nget added to the aggregator. <a href=\"radix_engine/types/trait.Describe.html#method.add_all_dependencies\">Read more</a></div></details></div></details>","Describe<ScryptoCustomTypeKind>","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-AccountSubstate\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#impl-PartialEq-for-AccountSubstate\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.PartialEq.html\" title=\"trait radix_engine::types::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/blueprints/account/blueprint.rs.html#33\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"radix_engine/blueprints/account/blueprint/struct.AccountSubstate.html\" title=\"struct radix_engine::blueprints::account::blueprint::AccountSubstate\">AccountSubstate</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","radix_engine::blueprints::account::blueprint::AccountDepositRuleV1"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()