// License Generator - Frontend

const invoke = window.__TAURI__?.core?.invoke;
const dialog = window.__TAURI__?.dialog;

// ── DOM cache ──
const $ = {};
function cacheDom() {
  $.navItems = document.querySelectorAll('.nav-item');
  $.pages = document.querySelectorAll('.page');
  $.macInput = document.getElementById('macInput');
  $.detectMacBtn = document.getElementById('detectMacBtn');
  $.expiryNever = document.getElementById('expiryNever');
  $.expiryCustom = document.getElementById('expiryCustom');
  $.dateRange = document.getElementById('dateRange');
  $.expiryDate = document.getElementById('expiryDate');
  $.chkFactory = document.getElementById('chkFactory');
  $.chkFirmware = document.getElementById('chkFirmware');
  $.licenseeInput = document.getElementById('licenseeInput');
  $.noteInput = document.getElementById('noteInput');
  $.generateBtn = document.getElementById('generateBtn');
  $.outputArea = document.getElementById('outputArea');
  $.statusBar = document.getElementById('statusBar');
  $.licensePreview = document.getElementById('licensePreview');
  $.verifyPath = document.getElementById('verifyPath');
  $.pickVerifyBtn = document.getElementById('pickVerifyBtn');
  $.verifyBtn = document.getElementById('verifyBtn');
  $.verifyResult = document.getElementById('verifyResult');
  $.verifyResultText = document.getElementById('verifyResultText');
  $.toast = document.getElementById('toast');
}

// ── Navigation ──
function initNav() {
  document.querySelector('.nav').addEventListener('click', (e) => {
    const item = e.target.closest('.nav-item');
    if (!item) return;
    const page = item.dataset.page;
    if (!page) return;
    $.navItems.forEach(n => n.classList.remove('active'));
    item.classList.add('active');
    $.pages.forEach(p => p.classList.remove('active'));
    document.getElementById('page-' + page).classList.add('active');
  });
}

// ── Toast ──
let toastTimer = null;
function showToast(msg, type='info') {
  if (toastTimer) clearTimeout(toastTimer);
  $.toast.textContent = msg;
  $.toast.className = 'toast ' + type + ' show';
  toastTimer = setTimeout(() => {
    $.toast.classList.remove('show');
  }, 3000);
}

// ── MAC detection ──
async function detectMac() {
  try {
    const macs = await invoke('get_mac_addresses');
    if (macs && macs.length > 0) {
      $.macInput.value = macs[0];
      showToast('已检测到本机 MAC 地址', 'success');
    } else {
      showToast('未检测到网卡', 'error');
    }
  } catch (e) {
    showToast('检测失败: ' + e, 'error');
  }
}

// ── Expiry toggle ──
function initExpiry() {
  $.expiryNever.addEventListener('change', () => {
    $.dateRange.style.display = 'none';
  });
  $.expiryCustom.addEventListener('change', () => {
    $.dateRange.style.display = 'block';
    const d = new Date();
    d.setFullYear(d.getFullYear() + 1);
    $.expiryDate.value = d.toISOString().split('T')[0];
  });
  // Set default date
  const d = new Date();
  d.setFullYear(d.getFullYear() + 1);
  $.expiryDate.value = d.toISOString().split('T')[0];
}

// ── Generate ──
async function generate() {
  const mac = $.macInput.value.trim();
  if (!mac) { showToast('请输入 MAC 地址', 'error'); return; }
  if (!/^[0-9A-Fa-f]{12}$/.test(mac)) {
    showToast('MAC 地址格式错误（应为 12 位十六进制字符）', 'error');
    return;
  }

  const expiresAt = $.expiryNever.checked ? '0' : ($.expiryDate.value || '0');
  if (expiresAt !== '0' && !expiresAt) {
    showToast('请选择有效期截止日期', 'error');
    return;
  }

  $.generateBtn.disabled = true;
  $.generateBtn.textContent = '正在生成...';

  try {
    const result = await invoke('generate_license', {
      req: {
        mac: mac.toUpperCase(),
        expires_at: expiresAt,
        factory_mode: $.chkFactory.checked,
        firmware_download: $.chkFirmware.checked,
        licensee: $.licenseeInput.value.trim(),
        note: $.noteInput.value.trim(),
      }
    });

    $.outputArea.style.display = 'block';
    if (result.success) {
      $.statusBar.className = 'status-msg success';
      $.statusBar.textContent = result.message;
      if (result.preview) {
        $.licensePreview.textContent = result.preview;
      }
      showToast(result.message, 'success');
    } else {
      $.statusBar.className = 'status-msg error';
      $.statusBar.textContent = result.message;
      if (result.preview) {
        $.licensePreview.textContent = result.preview;
      }
      showToast(result.message, 'info');
    }
  } catch (e) {
    const errMsg = String(e);
    $.outputArea.style.display = 'block';
    $.statusBar.className = 'status-msg error';
    $.statusBar.textContent = '';
    $.licensePreview.textContent = '';
    if (errMsg.includes('未找到私钥文件')) {
      showToast(
        '私钥文件缺失\n\n请将 modem-cat.sk 放到以下位置之一：\n' +
        '1. keys/modem-cat.sk（开发模式）\n' +
        '2. <exe_dir>/keys/modem-cat.sk（生产模式）\n' +
        '3. 或设置环境变量 MODEM_CAT_SK_PATH 指向私钥文件',
        'error'
      );
    } else {
      showToast('生成失败: ' + e, 'error');
    }
  } finally {
    $.generateBtn.disabled = false;
    $.generateBtn.textContent = '生成 License 文件';
  }
}

// ── Verify ──
async function pickVerifyFile() {
  try {
    const selected = await dialog.open({
      multiple: false,
      filters: [{ name: 'License File', extensions: ['dat'] }, { name: 'All Files', extensions: ['*'] }]
    });
    if (selected && selected.length > 0) {
      $.verifyPath.value = selected[0];
    }
  } catch (e) {
    showToast('选择文件失败: ' + e, 'error');
  }
}

async function verifyLicense() {
  const path = $.verifyPath.value.trim();
  if (!path) { showToast('请输入或选择 License 文件路径', 'error'); return; }

  $.verifyBtn.disabled = true;
  $.verifyBtn.textContent = '验证中...';

  try {
    const result = await invoke('verify_license_file', { path });
    $.verifyResult.style.display = 'block';
    $.verifyResultText.textContent = result;
    if (result.startsWith('✅')) {
      showToast('License 有效', 'success');
    } else {
      showToast('License 无效', 'error');
    }
  } catch (e) {
    $.verifyResult.style.display = 'block';
    $.verifyResultText.textContent = '验证出错: ' + e;
    showToast('验证失败: ' + e, 'error');
  } finally {
    $.verifyBtn.disabled = false;
    $.verifyBtn.textContent = '验证';
  }
}

// ── Init ──
function init() {
  cacheDom();
  initNav();
  initExpiry();

  $.detectMacBtn.addEventListener('click', detectMac);
  $.generateBtn.addEventListener('click', generate);
  $.pickVerifyBtn.addEventListener('click', pickVerifyFile);
  $.verifyBtn.addEventListener('click', verifyLicense);

  // Auto-detect MAC on load
  detectMac();

  // Export public key for main app (console only, for debugging/setup)
  exportPublicKey().catch(() => {});
}

async function exportPublicKey() {
  try {
    const pubKey = await invoke('export_public_key');
    console.log('%c=== PUBLIC KEY FOR MAIN APP ===', 'color: #f97316; font-weight: bold;');
    console.log(pubKey);
    console.log('%cCopy the above into modem-license/src/lib.rs as PUBLIC_KEY_BYTES', 'color: #94a3b8;');
  } catch (e) {
    console.error('Failed to export public key:', e);
  }
}

document.addEventListener('DOMContentLoaded', init);
