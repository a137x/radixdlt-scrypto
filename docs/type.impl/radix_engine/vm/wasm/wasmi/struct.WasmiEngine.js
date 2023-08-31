(function() {var type_impls = {
"radix_engine":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-WasmiEngine\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/vm/wasm/wasmi.rs.html#1832-1851\">source</a><a href=\"#impl-WasmiEngine\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"radix_engine/vm/wasm/wasmi/struct.WasmiEngine.html\" title=\"struct radix_engine::vm::wasm::wasmi::WasmiEngine\">WasmiEngine</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/radix_engine/vm/wasm/wasmi.rs.html#1833-1850\">source</a><h4 class=\"code-header\">pub fn <a href=\"radix_engine/vm/wasm/wasmi/struct.WasmiEngine.html#tymethod.new\" class=\"fn\">new</a>(options: <a class=\"struct\" href=\"radix_engine/vm/wasm/wasmi/struct.WasmiEngineOptions.html\" title=\"struct radix_engine::vm::wasm::wasmi::WasmiEngineOptions\">WasmiEngineOptions</a>) -&gt; Self</h4></section></div></details>",0,"radix_engine::vm::wasm::DefaultWasmEngine"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-WasmEngine-for-WasmiEngine\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/vm/wasm/wasmi.rs.html#1853-1887\">source</a><a href=\"#impl-WasmEngine-for-WasmiEngine\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/vm/wasm/traits/trait.WasmEngine.html\" title=\"trait radix_engine::vm::wasm::traits::WasmEngine\">WasmEngine</a> for <a class=\"struct\" href=\"radix_engine/vm/wasm/wasmi/struct.WasmiEngine.html\" title=\"struct radix_engine::vm::wasm::wasmi::WasmiEngine\">WasmiEngine</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.WasmInstance\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.WasmInstance\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"radix_engine/vm/wasm/traits/trait.WasmEngine.html#associatedtype.WasmInstance\" class=\"associatedtype\">WasmInstance</a> = <a class=\"struct\" href=\"radix_engine/vm/wasm/wasmi/struct.WasmiInstance.html\" title=\"struct radix_engine::vm::wasm::wasmi::WasmiInstance\">WasmiInstance</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.instantiate\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/vm/wasm/wasmi.rs.html#1857-1886\">source</a><a href=\"#method.instantiate\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/vm/wasm/traits/trait.WasmEngine.html#tymethod.instantiate\" class=\"fn\">instantiate</a>(\n    &amp;self,\n    code_hash: <a class=\"struct\" href=\"radix_engine/types/blueprints/package/struct.CodeHash.html\" title=\"struct radix_engine::types::blueprints::package::CodeHash\">CodeHash</a>,\n    instrumented_code: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a>]\n) -&gt; <a class=\"struct\" href=\"radix_engine/vm/wasm/wasmi/struct.WasmiInstance.html\" title=\"struct radix_engine::vm::wasm::wasmi::WasmiInstance\">WasmiInstance</a></h4></section></summary><div class='docblock'>Instantiate a Scrypto module. <a href=\"radix_engine/vm/wasm/traits/trait.WasmEngine.html#tymethod.instantiate\">Read more</a></div></details></div></details>","WasmEngine","radix_engine::vm::wasm::DefaultWasmEngine"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-WasmiEngine\" class=\"impl\"><a class=\"src rightside\" href=\"src/radix_engine/vm/wasm/wasmi.rs.html#1824-1830\">source</a><a href=\"#impl-Default-for-WasmiEngine\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"radix_engine/types/trait.Default.html\" title=\"trait radix_engine::types::Default\">Default</a> for <a class=\"struct\" href=\"radix_engine/vm/wasm/wasmi/struct.WasmiEngine.html\" title=\"struct radix_engine::vm::wasm::wasmi::WasmiEngine\">WasmiEngine</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/radix_engine/vm/wasm/wasmi.rs.html#1825-1829\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"radix_engine/types/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"radix_engine/types/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","radix_engine::vm::wasm::DefaultWasmEngine"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()