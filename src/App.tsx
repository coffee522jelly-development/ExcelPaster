import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import "./App.css";

interface EvidencePair {
  key: string;
  left_image: string | null;
  right_image: string | null;
}

const ImagePreview = ({ path }: { path: string | null }) => {
  const [dataUrl, setDataUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!path) return;
    invoke<string>("read_file_base64", { path })
      .then((url) => setDataUrl(url))
      .catch((err) => console.error("Failed to load image", err));
  }, [path]);

  if (!dataUrl) return <span className="icon-ok">○</span>;
  return (
    <div className="status-cell">
      <span className="icon-ok">○</span>
      <img src={dataUrl} alt="Preview" className="thumbnail" />
    </div>
  );
};

interface ScanResult {
  pairs: EvidencePair[];
  errors: string[];
}

function App() {
  const [leftToken, setLeftToken] = useState("b");
  const [rightToken, setRightToken] = useState("a");
  const [leftHeader, setLeftHeader] = useState("改修前");
  const [rightHeader, setRightHeader] = useState("改修後");

  const [folderPath, setFolderPath] = useState<string | null>(null);
  const [pairs, setPairs] = useState<EvidencePair[]>([]);
  const [errors, setErrors] = useState<string[]>([]);
  const [isExporting, setIsExporting] = useState(false);
  const [showSettings, setShowSettings] = useState(false);

  const handleSelectFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected && typeof selected === "string") {
        setFolderPath(selected);
        scanDirectory(selected);
      }
    } catch (error) {
      console.error("Failed to select folder", error);
      setErrors([`フォルダの選択に失敗しました: ${error}`]);
    }
  };

  const scanDirectory = async (path: string) => {
    try {
      setErrors([]);
      const result: ScanResult = await invoke("scan_directory", {
        path,
        leftToken,
        rightToken,
      });
      setPairs(result.pairs);
      if (result.errors && result.errors.length > 0) {
        setErrors(result.errors);
      }
    } catch (error) {
      console.error("Scan failed", error);
      setErrors([`ディレクトリのスキャンに失敗しました: ${error}`]);
    }
  };

  const handleExportExcel = async () => {
    if (pairs.length === 0) {
      alert("エクスポートするデータがありません");
      return;
    }

    try {
      const savePath = await save({
        filters: [{ name: "Excel", extensions: ["xlsx"] }],
        defaultPath: "evidence.xlsx",
      });

      if (savePath) {
        setIsExporting(true);
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
      setIsExporting(false);
    }
  };

  return (
    <div className="container">
      <div className="header-panel">
        <button className="settings-button" onClick={() => setShowSettings(!showSettings)}>
          {showSettings ? "設定を閉じる" : "設定を開く"}
        </button>
      </div>

      {showSettings && (
        <div className="settings-panel">
          <h3>設定</h3>
          <div className="input-group">
            <label>左画像識別子</label>
            <input
              type="text"
              value={leftToken}
              onChange={(e) => {
                setLeftToken(e.target.value);
              }}
              onBlur={() => {
                if (folderPath) scanDirectory(folderPath);
              }}
            />
          </div>
          <div className="input-group">
            <label>右画像識別子</label>
            <input
              type="text"
              value={rightToken}
              onChange={(e) => {
                setRightToken(e.target.value);
              }}
              onBlur={() => {
                if (folderPath) scanDirectory(folderPath);
              }}
            />
          </div>
          <div className="input-group">
            <label>左画像エクセル文言</label>
            <input
              type="text"
              value={leftHeader}
              onChange={(e) => setLeftHeader(e.target.value)}
            />
          </div>
          <div className="input-group">
            <label>右画像エクセル文言</label>
            <input
              type="text"
              value={rightHeader}
              onChange={(e) => setRightHeader(e.target.value)}
            />
          </div>
        </div>
      )}

      <div className="action-panel">
        <button onClick={handleSelectFolder}>フォルダ選択</button>
        {folderPath && <span className="path-display">{folderPath}</span>}
      </div>

      {errors.length > 0 && (
        <div className="error-panel">
          {errors.map((err, i) => (
            <div key={i} className="error-message">
              {err}
            </div>
          ))}
        </div>
      )}

      <div className="list-panel">
        <h3>ペア一覧</h3>
        <table>
          <thead>
            <tr>
              <th>項目</th>
              <th>{leftHeader}</th>
              <th>{rightHeader}</th>
            </tr>
          </thead>
          <tbody>
            {pairs.map((pair) => (
              <tr key={pair.key}>
                <td>{pair.key}</td>
                <td className="center-text">
                  {pair.left_image ? (
                     <ImagePreview path={pair.left_image} />
                  ) : (
                    <span className="icon-ng">×</span>
                  )}
                </td>
                <td className="center-text">
                  {pair.right_image ? (
                     <ImagePreview path={pair.right_image} />
                  ) : (
                    <span className="icon-ng">×</span>
                  )}
                </td>
              </tr>
            ))}
            {pairs.length === 0 && (
              <tr>
                <td colSpan={3} className="center-text empty-msg">データがありません</td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      <div className="export-panel">
        <button onClick={handleExportExcel} disabled={isExporting || pairs.length === 0}>
          {isExporting ? "出力中..." : "Excel出力"}
        </button>
      </div>
    </div>
  );
}

export default App;
