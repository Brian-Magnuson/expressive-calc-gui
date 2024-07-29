<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let input = "";
  let output = "";

  async function handleSubmit(_: SubmitEvent) {
    output = (await invoke("evaluate_expression", {
      expression: input,
    })) as string;
  }

  async function handleClear() {
    input = "";
    output = "";
    await invoke("clear_expression");
  }

  async function handleClearEntry() {
    input = "";
  }
</script>

<main>
  <form on:submit|preventDefault={handleSubmit}>
    <input type="text" bind:value={input} />
    <button type="submit"> &equals; </button>
    <button type="button" on:click={handleClear}> C </button>
    <button type="button" on:click={handleClearEntry}> CE </button>
  </form>
  <div>{output}</div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
