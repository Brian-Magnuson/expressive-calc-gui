<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let inputElement: HTMLInputElement;

  onMount(() => {
    inputElement.focus();
  });

  let input = "";
  let outputs: { variable: string; expression: string; result: string }[] = [];

  async function handleSubmit() {
    const output = (await invoke("evaluate_expression", {
      expression: input,
    })) as string;

    const matches = output.match(/(\$\d+) = (\d+|NaN)/);
    if (matches) {
      const [, variable, result] = matches;
      outputs = [{ variable, expression: input, result }, ...outputs];
      input = "";
    } else {
      outputs = [{ variable: "", expression: "", result: output }, ...outputs];
    }
  }

  async function handleClear() {
    input = "";
    outputs = [];
    await invoke("reset_calculator");
  }
  type CalcButton = {
    s: string;
    f?: () => void;
  };

  const buttons: CalcButton[][] = [
    [
      { s: "⌫", f: () => (input = input.slice(0, -1)) },
      { s: "(" },
      { s: ")" },
      { s: "," },
      { s: "C", f: async () => await handleClear() },
    ],
    [
      { s: "7" },
      { s: "8" },
      { s: "9" },
      { s: "/" },
      { s: "sqrt", f: () => (input += "sqrt(") },
    ],
    [{ s: "4" }, { s: "5" }, { s: "6" }, { s: "*" }, { s: "^" }],
    [
      { s: "1" },
      { s: "2" },
      { s: "3" },
      { s: "-" },
      { s: "abs", f: () => (input += "abs(") },
    ],
    [
      { s: "0" },
      { s: "." },
      { s: "ans", f: () => (input += "$ans") },
      { s: "+" },
      { s: "=", f: async () => await handleSubmit() },
    ],
  ];
</script>

<main>
  <div class="container">
    <div class="upper">
      {#each outputs as output}
        <div class="entry">
          <span class="entry-variable">{output.variable}</span>
          <span>{output.expression}</span>
          <span>= {output.result}</span>
        </div>
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
        {#each buttons.flat() as { s, f }}
          <button
            type="button"
            on:click|preventDefault={f ? f : () => (input += s)}>{s}</button
          >
        {/each}
      </div>
    </form>
  </div>
</main>

<style>
  * {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 24px;
    line-height: 24px;
    font-weight: 400;

    --base-100: #f6f6f6;
    --base-200: #eeeeee;
    --base-300: #dfdfdf;

    color: #0f0f0f;
    background-color: var(--base-200);

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
    justify-content: center;
    align-items: center;
  }

  .container {
    /* align */
    height: 100%;
    width: 100%;
    max-width: 800px;
    /* content */
    padding: 4rem 1rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .upper {
    /* align */
    flex: 1;
    min-height: 5rem;
    width: 100%;
    /* style */
    border: 1px solid var(--base-300);
    border-bottom: none;
    border-radius: 0.25rem 0.25rem 0 0;
    overflow-y: auto;
    /* background: blue; */
    /* content */
    display: flex;
    flex-direction: column-reverse;
  }

  .upper .entry {
    /* align */
    height: 46px;
    /* style */
    text-align: right;
    border-top: 1px solid var(--base-300);
    /* content */
    padding: 0.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .upper .entry-variable {
    /* align */
    /* style */
    font-size: 0.6rem;
  }

  .lower {
    /* align */
    width: 100%;
  }

  .lower .input {
    /* align */
    display: block;
    width: 100%;
    margin-bottom: 1rem;
    /* style */
    color: inherit;
    border: 1px solid var(--base-300);
    border-radius: 0 0 0.25rem 0.25rem;
    background: var(--base-100);
    font-size: 24px;
    /* content */
    padding: 0.5rem;
  }

  .lower .button-grid {
    /* content */
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    grid-auto-rows: minmax(40px, auto);
    gap: 4px;
  }

  .lower .button-grid button {
    /* style */
    font-size: 1rem;
    color: inherit;
    background-color: var(--base-100);
    border: 1px solid var(--base-300);
    border-radius: 0.25rem;
  }

  .lower .button-grid button:hover {
    /* style */
    background-color: var(--base-200);
    cursor: pointer;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      --base-100: #2f2f2f;
      --base-200: #2a2a2a;
      --base-300: #212121;

      color: #f6f6f6;
    }
  }
</style>
