<script lang="ts">
  import type { DndEvent } from "svelte-dnd-action";
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { GripVertical } from "lucide-svelte";
  import { dndzone } from "svelte-dnd-action";
  import ImagePreview from "./lib/ImagePreview.svelte";
  import SettingsPanel from "./lib/SettingsPanel.svelte";
  import ThemeToggle from "./lib/ThemeToggle.svelte";
  import "./app.css";

  interface EvidencePair {
    id?: string;
    key: string;
    left_image: string | null;
    right_image: string | null;
    extra_image: string | null;
  }

  interface ScanResult {
    pairs: EvidencePair[];
    errors: string[];
  }

  let leftToken = "_b";
  let rightToken = "_a";
  let extraToken = "_aa";
  let leftHeader = "改修前";
  let rightHeader = "改修後";
  let extraHeader = "補足";
  let leftColor = "#fed7aa"; // Default light orange
  let rightColor = "#bbf7d0"; // Default light green
  let extraColor = "#fef08a"; // Default light yellow

  const today = new Date();
  const initYy = String(today.getFullYear()).slice(-2);
  const initMm = String(today.getMonth() + 1).padStart(2, "0");
  const initDd = String(today.getDate()).padStart(2, "0");

  let sheetName = "検証（開発）";
  let subject = `${initYy}${initMm}${initDd}-`;

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

  async function handleBatchRename(suffix: string) {
    try {
      const selected = await open({
        multiple: true,
        filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "bmp"] }]
      });

      if (selected && Array.isArray(selected) && selected.length > 0) {
        await invoke("rename_files", {
          filePaths: selected,
          suffix: suffix
        });
        alert(`${selected.length}個のファイルに「${suffix}」を付与しました`);

        if (folderPath) {
          await scanDirectory(folderPath);
        }
      }
    } catch (error) {
      console.error("Failed to rename files", error);
      errors = [`一括リネームに失敗しました: ${error}`];
    }
  }

  async function scanDirectory(path: string) {
    try {
      errors = [];
      const result: ScanResult = await invoke("scan_directory", {
        path,
        leftToken,
        rightToken,
        extraToken,
      });
      pairs = result.pairs.map((p) => ({ ...p, id: p.key }));
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
          extraHeader,
          leftColor,
          rightColor,
          extraColor,
          sheetName,
          subject,
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

  function handleDndConsider(e: CustomEvent<DndEvent<EvidencePair>>) {
    pairs = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent<DndEvent<EvidencePair>>) {
    pairs = e.detail.items;
  }
</script>

<main class="max-w-4xl mx-auto p-2 flex flex-col gap-2 min-h-screen text-xs">

  <div class="flex justify-end items-center gap-1">
    <ThemeToggle />
    <SettingsPanel
      bind:showSettings
      bind:leftToken
      bind:rightToken
      bind:extraToken
      bind:leftHeader
      bind:rightHeader
      bind:extraHeader
      bind:leftColor
      bind:rightColor
      bind:extraColor
      onTokenBlur={handleTokenBlur}
    />
  </div>

  <div class="flex items-center gap-2 bg-white dark:bg-slate-900 p-2 rounded shadow-sm border border-slate-200 dark:border-slate-800">
    <label class="font-semibold text-slate-700 dark:text-slate-300 whitespace-nowrap" for="sheetNameTop">シート名:</label>
    <input
      id="sheetNameTop"
      type="text"
      class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none w-24"
      bind:value={sheetName}
    />
    <div class="w-px h-4 bg-slate-200 dark:bg-slate-700 mx-1"></div>
    <label class="font-semibold text-slate-700 dark:text-slate-300 whitespace-nowrap" for="subjectTop">主題:</label>
    <input
      id="subjectTop"
      type="text"
      class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none w-48"
      bind:value={subject}
    />
    <div class="w-px h-4 bg-slate-200 dark:bg-slate-700 mx-1 flex-shrink-0"></div>
    <button
      class="px-2 py-1 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded transition-colors whitespace-nowrap flex-shrink-0"
      on:click={handleSelectFolder}
    >
      フォルダ選択
    </button>

    <div class="w-px h-4 bg-slate-200 dark:bg-slate-700 mx-1 flex-shrink-0"></div>
    <span class="font-semibold text-slate-700 dark:text-slate-300 whitespace-nowrap">一括リネーム:</span>
    <button
      class="px-2 py-1 bg-slate-200 hover:bg-slate-300 dark:bg-slate-800 dark:hover:bg-slate-700 text-slate-700 dark:text-slate-300 font-semibold rounded transition-colors whitespace-nowrap flex-shrink-0"
      on:click={() => handleBatchRename(leftToken)}
    >
      左側識別子
    </button>
    <button
      class="px-2 py-1 bg-slate-200 hover:bg-slate-300 dark:bg-slate-800 dark:hover:bg-slate-700 text-slate-700 dark:text-slate-300 font-semibold rounded transition-colors whitespace-nowrap flex-shrink-0"
      on:click={() => handleBatchRename(rightToken)}
    >
      右側識別子
    </button>
  </div>

  {#if folderPath}
    <div class="px-2 text-slate-600 dark:text-slate-400 truncate text-xs" title={folderPath}>
      {folderPath}
    </div>
  {/if}

  {#if errors.length > 0}
    <div class="bg-red-50 dark:bg-red-950/50 text-red-700 dark:text-red-400 p-2 rounded border border-red-200 dark:border-red-900 flex flex-col gap-1">
      {#each errors as err}
        <div>{err}</div>
      {/each}
    </div>
  {/if}

  <div class="bg-white dark:bg-slate-900 p-2 rounded shadow-sm border border-slate-200 dark:border-slate-800 overflow-x-auto flex-1">
    <table class="w-full border-collapse text-left">
      <thead>
        <tr class="bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
          <th class="p-1 w-8"></th>
          <th class="p-1 font-semibold w-24 truncate">項目</th>
          <th class="p-1 font-semibold w-1/3">{leftHeader}</th>
          <th class="p-1 font-semibold w-1/3">{rightHeader}</th>
          <th class="p-1 font-semibold w-1/3">{extraHeader}</th>
        </tr>
      </thead>
      <tbody
        use:dndzone={{items: pairs, flipDurationMs: 300, dropTargetStyle: { outline: 'none' }}}
        on:consider={handleDndConsider}
        on:finalize={handleDndFinalize}
      >
        {#each pairs as pair (pair.id)}
          <tr
            class="border-b border-slate-100 dark:border-slate-800/50 hover:bg-slate-50 dark:hover:bg-slate-800/30 transition-colors bg-white dark:bg-slate-900"
          >
            <td class="p-1 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200">
              <div class="flex justify-center">
                <GripVertical size={16} />
              </div>
            </td>
            <td class="p-1 font-medium truncate" title={pair.key}>{pair.key}</td>
            <td class="p-1">
              {#if pair.left_image}
                <ImagePreview path={pair.left_image} />
              {:else}
                <span class="text-red-500 font-bold pl-1">×</span>
              {/if}
            </td>
            <td class="p-1">
              {#if pair.right_image}
                <ImagePreview path={pair.right_image} />
              {:else}
                <span class="text-red-500 font-bold pl-1">×</span>
              {/if}
            </td>
            <td class="p-1">
              {#if pair.extra_image}
                <ImagePreview path={pair.extra_image} />
              {:else}
                <span class="text-slate-400 pl-1">-</span>
              {/if}
            </td>
          </tr>
        {/each}
        {#if pairs.length === 0}
          <tr>
            <td colspan="5" class="text-center text-slate-400 dark:text-slate-600 p-4">データがありません</td>
          </tr>
        {/if}
      </tbody>
    </table>
  </div>

  <div class="flex justify-end pb-2">
    <button
      class="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded transition-colors disabled:bg-slate-300 dark:disabled:bg-slate-700 disabled:text-slate-500 dark:disabled:text-slate-500 disabled:cursor-not-allowed"
      on:click={handleExportExcel}
      disabled={isExporting || pairs.length === 0}
    >
      {isExporting ? "出力中..." : "Excel出力"}
    </button>
  </div>
</main>
