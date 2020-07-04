//! HTML Templates
pub const HTML_TEMPLATE: &'static str = r#"
<script type="module">
 import init, { run } from '${entry}';
 (async () => {
     await init();
     run()
 })();
</script>
"#;

pub const DEV_HTML_TEMPLATE: &'static str = r#"
<script type="module">
 import init, { run } from '${entry}';
 (async () => {
    // Set Websocket
    const uri = "ws://" + location.host + "/updater";
    const ws = new WebSocket(uri);
    ws.onmessage = (event) => {
        console.log(event);
        console.log(event.data);
        event.data === "update" && location.reload();
    }

    // Run APP
    await init();
    run()
 })();
</script>
"#;
