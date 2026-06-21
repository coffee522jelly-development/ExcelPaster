<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import ImagePreview from "./lib/ImagePreview.svelte";
  import SettingsPanel from "./lib/SettingsPanel.svelte";
  import "./app.css";

  interface EvidencePair {
    key: string;
    left_image: string | null;
    right_image: string | null;
  }

  interface ScanResult {
    pairs: EvidencePair[];
    errors: string[];
  }

  let leftToken = "b";
  let rightToken = "a";
  let leftHeader = "改修前";
  let rightHeader = "改修後";

  let folderPath: string | null = null;
  let pairs: EvidencePair[] = [];
  let errors: string[] = [];
  let isExporting = false;
  let showSettings = false;

  async function handleSelectFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected && typeof selected === "string") {
        folderPath = selected;
        await scanDirectory(selected);
      }
    } catch (error) {
      console.error("Failed to select folder", error);
      errors = [`フォルダの選択に失敗しました: ${error}`];
    }
  }

  async function scanDirectory(path: string) {
    try {
      errors = [];
      const result: ScanResult = await invoke("scan_directory", {
        path,
        leftToken,
        rightToken,
      });
      pairs = result.pairs;
      if (result.errors && result.errors.length > 0) {
        errors = result.errors;
      }
    } catch (error) {
      console.error("Scan failed", error);
      errors = [`ディレクトリのスキャンに失敗しました: ${error}`];
    }
  }

  function handleTokenBlur() {
    if (folderPath) {
      scanDirectory(folderPath);
    }
  }

  async function handleExportExcel() {
    if (pairs.length === 0) {
      alert("エクスポートするデータがありません");
      return;
    }

    try {
      const today = new Date();
      const yy = String(today.getFullYear()).slice(-2);
      const mm = String(today.getMonth() + 1).padStart(2, "0");
      const dd = String(today.getDate()).padStart(2, "0");
      const defaultFilename = `${yy}${mm}${dd}-検証シート.xlsx`;

      const savePath = await save({
        filters: [{ name: "Excel", extensions: ["xlsx"] }],
        defaultPath: defaultFilename,
      });

      if (savePath) {
        isExporting = true;
        await invoke("generate_excel", {
          savePath,
          pairs,
          leftHeader,
          rightHeader,
        });
        alert("Excelの出力が完了しました");
      }
    } catch (error) {
      console.error("Export failed", error);
      alert(`Excelの出力に失敗しました: ${error}`);
    } finally {
      isExporting = false;
    }
  }
</script>

<main class="max-w-3xl mx-auto p-5 flex flex-col gap-5 bg-gray-50 min-h-screen text-gray-800">

  <SettingsPanel
    bind:showSettings
    bind:leftToken
    bind:rightToken
    bind:leftHeader
    bind:rightHeader
    onTokenBlur={handleTokenBlur}
  />

  <div class="flex items-center gap-4 bg-white p-4 rounded-lg shadow">
    <button
      class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded transition-colors"
      on:click={handleSelectFolder}
    >
      フォルダ選択
    </button>
    {#if folderPath}
      <span class="text-sm text-gray-600 break-all">{folderPath}</span>
    {/if}
  </div>

  {#if errors.length > 0}
    <div class="bg-red-50 text-red-700 p-4 rounded-lg border border-red-200 flex flex-col gap-1">
      {#each errors as err}
        <div class="text-sm">{err}</div>
      {/each}
    </div>
  {/if}

  <div class="bg-white p-4 rounded-lg shadow overflow-x-auto">
    <h3 class="mt-0 mb-4 text-lg font-semibold">ペア一覧</h3>
    <table class="w-full border-collapse">
      <thead>
        <tr class="bg-gray-100">
          <th class="border border-gray-300 p-2 text-left">項目</th>
          <th class="border border-gray-300 p-2 text-left">{leftHeader}</th>
          <th class="border border-gray-300 p-2 text-left">{rightHeader}</th>
        </tr>
      </thead>
      <tbody>
        {#each pairs as pair (pair.key)}
          <tr>
            <td class="border border-gray-300 p-2">{pair.key}</td>
            <td class="border border-gray-300 p-2 text-center">
              {#if pair.left_image}
                <ImagePreview path={pair.left_image} />
              {:else}
                <span class="text-red-700 font-bold">×</span>
              {/if}
            </td>
            <td class="border border-gray-300 p-2 text-center">
              {#if pair.right_image}
                <ImagePreview path={pair.right_image} />
              {:else}
                <span class="text-red-700 font-bold">×</span>
              {/if}
            </td>
          </tr>
        {/each}
        {#if pairs.length === 0}
          <tr>
            <td colspan="3" class="text-center text-gray-400 p-8 border border-gray-300">データがありません</td>
          </tr>
        {/if}
      </tbody>
    </table>
  </div>

  <div class="flex justify-end mt-2">
    <button
      class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
      on:click={handleExportExcel}
      disabled={isExporting || pairs.length === 0}
    >
      {isExporting ? "出力中..." : "Excel出力"}
    </button>
  </div>
</main>
