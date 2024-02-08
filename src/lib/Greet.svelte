<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let print = "";

  async function init() {
    await invoke("init_db");
  }

  async function query() {
    print = await invoke("query_db");
  }
</script>

<div>
  <div class="flex space-x-4">
    <button type="button" class="btn variant-filled" on:click={init}>
      ADD
    </button>
    <button type="button" class="btn variant-filled" on:click={query}>
      SHOW
    </button>
  </div>
  <p>{print}</p>
</div>
