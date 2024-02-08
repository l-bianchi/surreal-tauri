<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let print = "";

  async function init() {
    await invoke("init_db", { name });
  }

  async function query() {
    print = await invoke("query_db");
  }
</script>

<div
  class="flex flex-col w-full h-full p-12 items-center justify-center space-y-8"
>
  <input
    class="input w-1/2"
    type="text"
    placeholder="Enter a name..."
    bind:value={name}
  />
  <div class="flex space-x-8">
    <button type="button" class="btn variant-filled" on:click={init}>
      ADD
    </button>
    <button type="button" class="btn variant-filled" on:click={query}>
      SHOW
    </button>
  </div>
  <p>{print}</p>
</div>
