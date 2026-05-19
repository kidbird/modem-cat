# APN Configuration Modal Popup Design

## Overview

Convert the APN configuration page from inline form editing to a centered floating modal popup. Both adding and editing APNs will use the same popup. A "新增 APN" button will be added below the APN list on the main page.

## Approach

**Method A: Pure CSS Overlay + JS toggle** — A hidden `.apn-modal-overlay` div in the DOM, toggled via JS `display` property. Consistent with the project's single-file, no-framework architecture.

## Design Details

### 1. Modal Structure (HTML)

Add a new `.apn-modal-overlay` container to `index.html`, containing:
- A semi-transparent dark backdrop (clicking it closes the modal)
- A centered `.apn-modal-card` with:
  - Title: "新增 APN" (add mode) or "编辑 APN" (edit mode), dynamically set by JS
  - Form fields (reusing existing field structure): APN name, username, password, auth type dropdown, IP type dropdown
  - Bottom button row: "确认" (save) + "取消" (close)

### 2. Main Page Changes

- **Remove** the existing inline form section (lines 954-992 of `index.html` — the div below `#apnList` containing the form fields and save/cancel buttons)
- **Add** a "新增 APN" button after `#apnList`, styled with existing `btn` classes
- APN list items (`apn-item`) keep their current structure; clicking a list item opens the modal in edit mode instead of populating the inline form

### 3. CSS Additions

New CSS classes added to `index.html`:
- `.apn-modal-overlay`: `position: fixed; inset: 0; z-index: 100; background: rgba(0,0,0,0.5); display: none;` — full-screen semi-transparent backdrop
- `.apn-modal-card`: centered card, `max-width: 420px; background: var(--bg-primary); border-radius: 8px; padding: 24px;` — inherits existing dark theme variables
- Form field styles reuse existing `.field-group`, `.field-label`, `input`, `select` CSS classes already defined in the file

### 4. JS Logic Changes

- `editApn(i)`: Change from filling inline form → filling modal form fields + showing modal overlay
- `openApnModal()`: New function — clears all form fields, sets title to "新增 APN", sets `editingApnIdx = -1`, shows modal overlay
- `closeApnModal()`: New function — hides modal overlay, optionally clears form
- `saveApn()`: After successful save, automatically calls `closeApnModal()` + `refreshApnList()`. Core IPC call (`invoke('set_apn_config', ...)`) unchanged
- "新增 APN" button: `onclick="openApnModal()"`
- Backdrop click: `onclick="closeApnModal()"`
- Escape key: optionally close modal on Escape keydown

### 5. Data Flow (Unchanged)

- `saveApn()` → `invoke('set_apn_config', { cid, contextType, apn, username, password, authType })` — unchanged
- `refreshApnList()` → `invoke('get_apn_list')` — unchanged
- Backend (lib.rs, modem-hal) — no changes required

### 6. Scope

- **Frontend only** — all changes in `src/desktop/index.html`
- **No backend changes**
- **No new dependencies**

## Out of Scope

- Tauri new window approach
- Animation transitions (can be added later as enhancement)
- Changes to the `setActiveApn()` function (existing local-only behavior is a separate issue)
- CID collision handling for new APNs