(function() {var type_impls = {
"radix_engine":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#42\">source</a><a href=\"#impl-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.as_u8\" class=\"method\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#43\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine/types/enum.ValueKind.html#tymethod.as_u8\" class=\"fn\">as_u8</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.from_u8\" class=\"method\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#65\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine/types/enum.ValueKind.html#tymethod.from_u8\" class=\"fn\">from_u8</a>(id: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a>) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Option.html\" title=\"enum radix_engine::types::Option\">Option</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;&gt;</h4></section></div></details>",0,"radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CScryptoCustomValueKind%3E-for-ValueKind%3CScryptoCustomValueKind%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_common/data/scrypto/custom_value_kind.rs.html#26\">source</a><a href=\"#impl-From%3CScryptoCustomValueKind%3E-for-ValueKind%3CScryptoCustomValueKind%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.From.html\" title=\"trait radix_engine::types::From\">From</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt; for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_common/data/scrypto/custom_value_kind.rs.html#27\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.From.html#tymethod.from\" class=\"fn\">from</a>(\n    custom_value_kind: <a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>\n) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ScryptoCustomValueKind.html\" title=\"enum radix_engine::types::ScryptoCustomValueKind\">ScryptoCustomValueKind</a>&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<ScryptoCustomValueKind>","radix_engine::types::ScryptoValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CManifestCustomValueKind%3E-for-ValueKind%3CManifestCustomValueKind%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine_common/data/manifest/custom_value_kind.rs.html#34\">source</a><a href=\"#impl-From%3CManifestCustomValueKind%3E-for-ValueKind%3CManifestCustomValueKind%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.From.html\" title=\"trait radix_engine::types::From\">From</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ManifestCustomValueKind.html\" title=\"enum radix_engine::types::ManifestCustomValueKind\">ManifestCustomValueKind</a>&gt; for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ManifestCustomValueKind.html\" title=\"enum radix_engine::types::ManifestCustomValueKind\">ManifestCustomValueKind</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine_common/data/manifest/custom_value_kind.rs.html#35\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.From.html#tymethod.from\" class=\"fn\">from</a>(\n    custom_value_kind: <a class=\"enum\" href=\"radix_engine/types/enum.ManifestCustomValueKind.html\" title=\"enum radix_engine::types::ManifestCustomValueKind\">ManifestCustomValueKind</a>\n) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ManifestCustomValueKind.html\" title=\"enum radix_engine::types::ManifestCustomValueKind\">ManifestCustomValueKind</a>&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<ManifestCustomValueKind>","radix_engine::types::ManifestValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#impl-PartialEq-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"radix_engine/types/trait.PartialEq.html\" title=\"trait radix_engine::types::PartialEq\">PartialEq</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.PartialEq.html\" title=\"trait radix_engine::types::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#impl-Debug-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"radix_engine/types/trait.Debug.html\" title=\"trait radix_engine::types::Debug\">Debug</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.Debug.html\" title=\"trait radix_engine::types::Debug\">Debug</a> + <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"radix_engine/types/radix_engine_common/prelude/fmt/struct.Formatter.html\" title=\"struct radix_engine::types::radix_engine_common::prelude::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Result.html\" title=\"enum radix_engine::types::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"radix_engine/types/radix_engine_common/prelude/fmt/struct.Error.html\" title=\"struct radix_engine::types::radix_engine_common::prelude::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"radix_engine/types/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<section id=\"impl-StructuralPartialEq-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#impl-StructuralPartialEq-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"radix_engine/types/radix_engine_common/prelude/rust/marker/trait.StructuralPartialEq.html\" title=\"trait radix_engine::types::radix_engine_common::prelude::rust::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section>","StructuralPartialEq","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<section id=\"impl-Copy-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#impl-Copy-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"radix_engine/types/trait.Copy.html\" title=\"trait radix_engine::types::Copy\">Copy</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.Copy.html\" title=\"trait radix_engine::types::Copy\">Copy</a> + <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section>","Copy","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#33\">source</a><a href=\"#impl-Display-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"radix_engine/types/trait.Display.html\" title=\"trait radix_engine::types::Display\">Display</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#34\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"radix_engine/types/radix_engine_common/prelude/fmt/struct.Formatter.html\" title=\"struct radix_engine::types::radix_engine_common::prelude::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Result.html\" title=\"enum radix_engine::types::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"radix_engine/types/radix_engine_common/prelude/fmt/struct.Error.html\" title=\"struct radix_engine::types::radix_engine_common::prelude::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"radix_engine/types/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#9\">source</a><a href=\"#impl-Deserialize%3C'de%3E-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, X&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#9\">source</a><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(\n    __deserializer: __D\n) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Result.html\" title=\"enum radix_engine::types::Result\">Result</a>&lt;<a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;, &lt;__D as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</div></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"https://docs.rs/serde/1.0.197/serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Serialize-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#9\">source</a><a href=\"#impl-Serialize-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#9\">source</a><a href=\"#method.serialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serialize.html#tymethod.serialize\" class=\"fn\">serialize</a>&lt;__S&gt;(\n    &amp;self,\n    __serializer: __S\n) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.Result.html\" title=\"enum radix_engine::types::Result\">Result</a>&lt;&lt;__S as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, &lt;__S as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</div></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer. <a href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serialize.html#tymethod.serialize\">Read more</a></div></details></div></details>","Serialize","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<section id=\"impl-Eq-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#impl-Eq-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"radix_engine/types/trait.Eq.html\" title=\"trait radix_engine::types::Eq\">Eq</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.Eq.html\" title=\"trait radix_engine::types::Eq\">Eq</a> + <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section>","Eq","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-ValueKind%3CX%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#impl-Clone-for-ValueKind%3CX%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X&gt; <a class=\"trait\" href=\"radix_engine/types/trait.Clone.html\" title=\"trait radix_engine::types::Clone\">Clone</a> for <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;<div class=\"where\">where\n    X: <a class=\"trait\" href=\"radix_engine/types/trait.Clone.html\" title=\"trait radix_engine::types::Clone\">Clone</a> + <a class=\"trait\" href=\"radix_engine/types/trait.CustomValueKind.html\" title=\"trait radix_engine::types::CustomValueKind\">CustomValueKind</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sbor/value_kind.rs.html#12\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"enum\" href=\"radix_engine/types/enum.ValueKind.html\" title=\"enum radix_engine::types::ValueKind\">ValueKind</a>&lt;X&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"radix_engine/types/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"radix_engine/types/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","radix_engine::types::ManifestValueKind","radix_engine::types::ScryptoValueKind","radix_engine::types::BasicValueKind"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()