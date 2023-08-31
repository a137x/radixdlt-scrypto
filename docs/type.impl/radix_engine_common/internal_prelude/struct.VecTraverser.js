(function() {var type_impls = {
"radix_engine_common":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VecTraverser%3C'de,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/traversal/untyped/traverser.rs.html#157\">source</a><a href=\"#impl-VecTraverser%3C'de,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, T&gt; <a class=\"struct\" href=\"radix_engine_common/internal_prelude/struct.VecTraverser.html\" title=\"struct radix_engine_common::internal_prelude::VecTraverser\">VecTraverser</a>&lt;'de, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"radix_engine_common/internal_prelude/trait.CustomTraversal.html\" title=\"trait radix_engine_common::internal_prelude::CustomTraversal\">CustomTraversal</a>,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/sbor/traversal/untyped/traverser.rs.html#158-163\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_common/internal_prelude/struct.VecTraverser.html#tymethod.new\" class=\"fn\">new</a>(\n    input: &amp;'de [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a>],\n    max_depth: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.usize.html\">usize</a>,\n    expected_start: <a class=\"enum\" href=\"radix_engine_common/internal_prelude/enum.ExpectedStart.html\" title=\"enum radix_engine_common::internal_prelude::ExpectedStart\">ExpectedStart</a>&lt;&lt;T as <a class=\"trait\" href=\"radix_engine_common/internal_prelude/trait.CustomTraversal.html\" title=\"trait radix_engine_common::internal_prelude::CustomTraversal\">CustomTraversal</a>&gt;::<a class=\"associatedtype\" href=\"radix_engine_common/internal_prelude/trait.CustomTraversal.html#associatedtype.CustomValueKind\" title=\"type radix_engine_common::internal_prelude::CustomTraversal::CustomValueKind\">CustomValueKind</a>&gt;,\n    check_exact_end: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a>\n) -&gt; <a class=\"struct\" href=\"radix_engine_common/internal_prelude/struct.VecTraverser.html\" title=\"struct radix_engine_common::internal_prelude::VecTraverser\">VecTraverser</a>&lt;'de, T&gt;</h4></section><section id=\"method.next_event\" class=\"method\"><a class=\"src rightside\" href=\"src/sbor/traversal/untyped/traverser.rs.html#179\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine_common/internal_prelude/struct.VecTraverser.html#tymethod.next_event\" class=\"fn\">next_event</a>&lt;'t&gt;(&amp;'t mut self) -&gt; <a class=\"struct\" href=\"radix_engine_common/internal_prelude/struct.LocatedTraversalEvent.html\" title=\"struct radix_engine_common::internal_prelude::LocatedTraversalEvent\">LocatedTraversalEvent</a>&lt;'t, 'de, T&gt;</h4></section></div></details>",0,"radix_engine_common::data::manifest::definitions::ManifestTraverser","radix_engine_common::data::scrypto::definitions::ScryptoTraverser","radix_engine_common::prelude::BasicTraverser"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()