<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let inputElement: HTMLInputElement;

  onMount(() => {
    inputElement.focus();
  });

  let input = "";
  let outputs: string[] = [];

  async function handleSubmit(_: SubmitEvent) {
    const output = (await invoke("evaluate_expression", {
      expression: input,
    })) as string;

    input = "";
    outputs = [`${output}`, ...outputs];
  }

  async function handleClear() {
    input = "";
    outputs = [];
    await invoke("reset_calculator");
  }

  async function handleClearEntry() {
    input = "";
  }

  const symbols = [
    "7",
    "8",
    "9",
    "/",
    "4",
    "5",
    "6",
    "*",
    "1",
    "2",
    "3",
    "-",
    "0",
    ".",
    "+",
  ];
</script>

<main>
  <div class="upper">
    {#each outputs as output}
      <div class="entry">{output}</div>
    {/each}
  </div>
  <form class="lower" on:submit|preventDefault={handleSubmit}>
    <input
      class="input"
      type="text"
      bind:this={inputElement}
      bind:value={input}
    />
    <div class="button-grid">
      <button type="submit"> &equals; </button>
      <button type="button" on:click={handleClear}> C </button>
      <button type="button" on:click={handleClearEntry}> CE </button>
      <button type="button" on:click={() => (input = input.slice(0, -1))}>
        &larr;
      </button>
      {#each symbols as symbol}
        <button type="button" on:click={() => (input += symbol)}>
          {symbol}
        </button>
      {/each}
    </div>
  </form>
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

  main {
    /* align */
    height: 100vh;
    width: 100vw;
    /* style */
    /* background: red; */
    /* content */
    display: flex;
    flex-direction: column;
  }

  .upper {
    /* align */
    flex: 1;
    /* style */
    /* background: blue; */
    /* content */
    display: flex;
    flex-direction: column-reverse;
  }

  .upper .entry {
    /* style */
    text-align: right;
    border-top: 1px solid #ccc;
    /* content */
    padding: 0.5rem;
  }

  .lower {
    /* align */
    flex: 1.5;
  }

  .lower .input {
    /* align */
    display: block;
    box-sizing: border-box;
    width: 100%;
    /* style */
    border-radius: 0;
    /* content */
    padding: 0.5rem;
  }

  .lower .button-grid {
    /* content */
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-auto-rows: minmax(40px, auto);
  }

  .lower .button-grid button {
    /* style */
    font-size: 1.3rem;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
