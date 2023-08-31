(function() {var type_impls = {
"sbor":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Entry%3C'a,+K,+V%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2618\">source</a><a href=\"#impl-Entry%3C'a,+K,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, K, V&gt; <a class=\"enum\" href=\"sbor/prelude/hash_map/enum.Entry.html\" title=\"enum sbor::prelude::hash_map::Entry\">Entry</a>&lt;'a, K, V&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.or_insert\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2637\">source</a></span><h4 class=\"code-header\">pub fn <a href=\"sbor/prelude/hash_map/enum.Entry.html#tymethod.or_insert\" class=\"fn\">or_insert</a>(self, default: V) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;'a mut V</a></h4></section></summary><div class=\"docblock\"><p>Ensures a value is in the entry by inserting the default if empty, and returns\na mutable reference to the value in the entry.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::collections::HashMap;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>map: HashMap&lt;<span class=\"kw-2\">&amp;</span>str, u32&gt; = HashMap::new();\n\nmap.entry(<span class=\"string\">\"poneyland\"</span>).or_insert(<span class=\"number\">3</span>);\n<span class=\"macro\">assert_eq!</span>(map[<span class=\"string\">\"poneyland\"</span>], <span class=\"number\">3</span>);\n\n<span class=\"kw-2\">*</span>map.entry(<span class=\"string\">\"poneyland\"</span>).or_insert(<span class=\"number\">10</span>) <span class=\"kw-2\">*</span>= <span class=\"number\">2</span>;\n<span class=\"macro\">assert_eq!</span>(map[<span class=\"string\">\"poneyland\"</span>], <span class=\"number\">6</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.or_insert_with\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2661\">source</a></span><h4 class=\"code-header\">pub fn <a href=\"sbor/prelude/hash_map/enum.Entry.html#tymethod.or_insert_with\" class=\"fn\">or_insert_with</a>&lt;F&gt;(self, default: F) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;'a mut V</a><div class=\"where\">where\n    F: <a class=\"trait\" href=\"sbor/prelude/trait.FnOnce.html\" title=\"trait sbor::prelude::FnOnce\">FnOnce</a>() -&gt; V,</div></h4></section></summary><div class=\"docblock\"><p>Ensures a value is in the entry by inserting the result of the default function if empty,\nand returns a mutable reference to the value in the entry.</p>\n<h5 id=\"examples-1\"><a class=\"doc-anchor\" href=\"#examples-1\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::collections::HashMap;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>map = HashMap::new();\n<span class=\"kw\">let </span>value = <span class=\"string\">\"hoho\"</span>;\n\nmap.entry(<span class=\"string\">\"poneyland\"</span>).or_insert_with(|| value);\n\n<span class=\"macro\">assert_eq!</span>(map[<span class=\"string\">\"poneyland\"</span>], <span class=\"string\">\"hoho\"</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.or_insert_with_key\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.50.0\">1.50.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2688\">source</a></span><h4 class=\"code-header\">pub fn <a href=\"sbor/prelude/hash_map/enum.Entry.html#tymethod.or_insert_with_key\" class=\"fn\">or_insert_with_key</a>&lt;F&gt;(self, default: F) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;'a mut V</a><div class=\"where\">where\n    F: <a class=\"trait\" href=\"sbor/prelude/trait.FnOnce.html\" title=\"trait sbor::prelude::FnOnce\">FnOnce</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;K</a>) -&gt; V,</div></h4></section></summary><div class=\"docblock\"><p>Ensures a value is in the entry by inserting, if empty, the result of the default function.\nThis method allows for generating key-derived values for insertion by providing the default\nfunction a reference to the key that was moved during the <code>.entry(key)</code> method call.</p>\n<p>The reference to the moved key is provided so that cloning or copying the key is\nunnecessary, unlike with <code>.or_insert_with(|| ... )</code>.</p>\n<h5 id=\"examples-2\"><a class=\"doc-anchor\" href=\"#examples-2\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::collections::HashMap;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>map: HashMap&lt;<span class=\"kw-2\">&amp;</span>str, usize&gt; = HashMap::new();\n\nmap.entry(<span class=\"string\">\"poneyland\"</span>).or_insert_with_key(|key| key.chars().count());\n\n<span class=\"macro\">assert_eq!</span>(map[<span class=\"string\">\"poneyland\"</span>], <span class=\"number\">9</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.key\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.10.0\">1.10.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2710\">source</a></span><h4 class=\"code-header\">pub fn <a href=\"sbor/prelude/hash_map/enum.Entry.html#tymethod.key\" class=\"fn\">key</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;K</a></h4></section></summary><div class=\"docblock\"><p>Returns a reference to this entry’s key.</p>\n<h5 id=\"examples-3\"><a class=\"doc-anchor\" href=\"#examples-3\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::collections::HashMap;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>map: HashMap&lt;<span class=\"kw-2\">&amp;</span>str, u32&gt; = HashMap::new();\n<span class=\"macro\">assert_eq!</span>(map.entry(<span class=\"string\">\"poneyland\"</span>).key(), <span class=\"kw-2\">&amp;</span><span class=\"string\">\"poneyland\"</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.and_modify\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.26.0\">1.26.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2739-2741\">source</a></span><h4 class=\"code-header\">pub fn <a href=\"sbor/prelude/hash_map/enum.Entry.html#tymethod.and_modify\" class=\"fn\">and_modify</a>&lt;F&gt;(self, f: F) -&gt; <a class=\"enum\" href=\"sbor/prelude/hash_map/enum.Entry.html\" title=\"enum sbor::prelude::hash_map::Entry\">Entry</a>&lt;'a, K, V&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"sbor/prelude/trait.FnOnce.html\" title=\"trait sbor::prelude::FnOnce\">FnOnce</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;mut V</a>),</div></h4></section></summary><div class=\"docblock\"><p>Provides in-place mutable access to an occupied entry before any\npotential inserts into the map.</p>\n<h5 id=\"examples-4\"><a class=\"doc-anchor\" href=\"#examples-4\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::collections::HashMap;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>map: HashMap&lt;<span class=\"kw-2\">&amp;</span>str, u32&gt; = HashMap::new();\n\nmap.entry(<span class=\"string\">\"poneyland\"</span>)\n   .and_modify(|e| { <span class=\"kw-2\">*</span>e += <span class=\"number\">1 </span>})\n   .or_insert(<span class=\"number\">42</span>);\n<span class=\"macro\">assert_eq!</span>(map[<span class=\"string\">\"poneyland\"</span>], <span class=\"number\">42</span>);\n\nmap.entry(<span class=\"string\">\"poneyland\"</span>)\n   .and_modify(|e| { <span class=\"kw-2\">*</span>e += <span class=\"number\">1 </span>})\n   .or_insert(<span class=\"number\">42</span>);\n<span class=\"macro\">assert_eq!</span>(map[<span class=\"string\">\"poneyland\"</span>], <span class=\"number\">43</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.insert_entry\" class=\"method\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2767\">source</a><h4 class=\"code-header\">pub fn <a href=\"sbor/prelude/hash_map/enum.Entry.html#tymethod.insert_entry\" class=\"fn\">insert_entry</a>(self, value: V) -&gt; <a class=\"struct\" href=\"sbor/prelude/hash_map/struct.OccupiedEntry.html\" title=\"struct sbor::prelude::hash_map::OccupiedEntry\">OccupiedEntry</a>&lt;'a, K, V&gt;</h4></section><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>entry_insert</code>)</span></div></span></summary><div class=\"docblock\"><p>Sets the value of the entry, and returns an <code>OccupiedEntry</code>.</p>\n<h5 id=\"examples-5\"><a class=\"doc-anchor\" href=\"#examples-5\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"attr\">#![feature(entry_insert)]\n</span><span class=\"kw\">use </span>std::collections::HashMap;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>map: HashMap&lt;<span class=\"kw-2\">&amp;</span>str, String&gt; = HashMap::new();\n<span class=\"kw\">let </span>entry = map.entry(<span class=\"string\">\"poneyland\"</span>).insert_entry(<span class=\"string\">\"hoho\"</span>.to_string());\n\n<span class=\"macro\">assert_eq!</span>(entry.key(), <span class=\"kw-2\">&amp;</span><span class=\"string\">\"poneyland\"</span>);</code></pre></div>\n</div></details></div></details>",0,"sbor::prelude::non_iter_map::Entry"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Entry%3C'a,+K,+V%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2778\">source</a><a href=\"#impl-Entry%3C'a,+K,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, K, V&gt; <a class=\"enum\" href=\"sbor/prelude/hash_map/enum.Entry.html\" title=\"enum sbor::prelude::hash_map::Entry\">Entry</a>&lt;'a, K, V&gt;<div class=\"where\">where\n    V: <a class=\"trait\" href=\"sbor/prelude/trait.Default.html\" title=\"trait sbor::prelude::Default\">Default</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.or_default\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.28.0\">1.28.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2796\">source</a></span><h4 class=\"code-header\">pub fn <a href=\"sbor/prelude/hash_map/enum.Entry.html#tymethod.or_default\" class=\"fn\">or_default</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.reference.html\">&amp;'a mut V</a></h4></section></summary><div class=\"docblock\"><p>Ensures a value is in the entry by inserting the default value if empty,\nand returns a mutable reference to the value in the entry.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::collections::HashMap;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>map: HashMap&lt;<span class=\"kw-2\">&amp;</span>str, <span class=\"prelude-ty\">Option</span>&lt;u32&gt;&gt; = HashMap::new();\nmap.entry(<span class=\"string\">\"poneyland\"</span>).or_default();\n\n<span class=\"macro\">assert_eq!</span>(map[<span class=\"string\">\"poneyland\"</span>], <span class=\"prelude-val\">None</span>);</code></pre></div>\n</div></details></div></details>",0,"sbor::prelude::non_iter_map::Entry"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Entry%3C'_,+K,+V%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.12.0\">1.12.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2089\">source</a></span><a href=\"#impl-Debug-for-Entry%3C'_,+K,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K, V&gt; <a class=\"trait\" href=\"sbor/prelude/trait.Debug.html\" title=\"trait sbor::prelude::Debug\">Debug</a> for <a class=\"enum\" href=\"sbor/prelude/hash_map/enum.Entry.html\" title=\"enum sbor::prelude::hash_map::Entry\">Entry</a>&lt;'_, K, V&gt;<div class=\"where\">where\n    K: <a class=\"trait\" href=\"sbor/prelude/trait.Debug.html\" title=\"trait sbor::prelude::Debug\">Debug</a>,\n    V: <a class=\"trait\" href=\"sbor/prelude/trait.Debug.html\" title=\"trait sbor::prelude::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.77.1/src/std/collections/hash/map.rs.html#2090\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sbor/prelude/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"sbor/prelude/fmt/struct.Formatter.html\" title=\"struct sbor::prelude::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"sbor/prelude/enum.Result.html\" title=\"enum sbor::prelude::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"sbor/prelude/fmt/struct.Error.html\" title=\"struct sbor::prelude::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"sbor/prelude/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","sbor::prelude::non_iter_map::Entry"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()