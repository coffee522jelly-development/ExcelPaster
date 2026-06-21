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
  <div class="flex flex-col items-center gap-1">
    <span class="text-green-700 font-bold">○</span>
    <img src={dataUrl} alt="Preview" class="max-w-[100px] max-h-[80px] border border-gray-200 rounded" />
  </div>
{:else}
  <span class="text-green-700 font-bold">○</span>
{/if}
