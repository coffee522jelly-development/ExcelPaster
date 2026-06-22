<script lang="ts">
  import { Sun, Moon } from "lucide-svelte";
  import { onMount } from "svelte";

  let isDark = false;

  onMount(() => {
    isDark = document.documentElement.classList.contains("dark");
  });

  function toggleTheme() {
    isDark = !isDark;
    if (isDark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }
</script>

<button
  class="p-1 text-slate-500 hover:text-slate-800 dark:text-slate-400 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors focus:outline-none"
  on:click={toggleTheme}
  aria-label="Toggle theme"
>
  {#if isDark}
    <Sun size={18} />
  {:else}
    <Moon size={18} />
  {/if}
</button>
