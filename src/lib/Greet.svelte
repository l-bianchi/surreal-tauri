<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }
</script>

<div>
  <div class="flex">
    <input
      class="input"
      type="text"
      placeholder="Enter a name..."
      bind:value={name}
    />
    <button type="button" class="btn variant-filled" on:click={greet}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>
