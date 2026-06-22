<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  export let path: string | null = null;
  let dataUrl: string | null = null;

  $: if (path) {
    invoke<string>('read_file_base64', { path })
      .then(url => {
        dataUrl = url;
      })
      .catch(err => {
        console.error("Failed to load image", err);
      });
  }
</script>

{#if dataUrl}
  <div class="flex items-center gap-2">
    <span class="text-green-600 dark:text-green-500 font-bold text-sm">○</span>
    <img src={dataUrl} alt="Preview" class="h-12 w-auto object-contain border border-slate-200 dark:border-slate-700 rounded bg-slate-50 dark:bg-slate-950" />
  </div>
{:else}
  <span class="text-green-600 dark:text-green-500 font-bold text-sm pl-1">○</span>
{/if}
