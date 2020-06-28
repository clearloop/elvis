//! HTML Template
pub const HTML_TEMPLATE: &'static str = r#"
<script type="module">
 import init, { run } from '${entry}';
 (async () => {
     await init();
     run()
 })();
</script>
"#;
