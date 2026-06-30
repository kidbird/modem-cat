# AT 指令功能映射

> 最近更新：2026-06-25
> 本文档只记录**当前正式主合同** AT 路径；历史兼容命令留在手册资料中，不作为 live 代码设计依据。

## 使用规则

- 实时状态查询失败必须直接报错，不要改发第二条 AT 掩盖失败。
- `Unknown` 型号的正式合同是直接报错，不允许猜测 adapter。
- 只有主合同命令写在本表；历史兼容命令请回原始手册查询，不视为 live 合同。

## 平台划分（按检测优先级）

| 优先级 | 厂商 | 关键字（CGMM 大写后子串匹配） | 代表型号 |
|---|---|---|---|
| 1 | **Qualcomm（高通）** | `RG500Q`, `RM500Q`, `RG520N`, `RM520N`, `RG525F`, `RG530F`, `RM530F`, `RM530N`, `RM551E`, `RM501Q`, `RG540F`, `RM540N` | RM520N, RM500Q, RG525F, RM530N, RM551E… |
| 2 | **ASR** | `RG255` | RG255AA |
| 3 | **UniSoc（展锐）** | `RG200U`, `RM500U`, `RG500U`, `RG501U`, `RM501U` | RM500U, RG200U, RG500U… |
| 4 | **Unknown** | 未识别型号 | — |

⚠️ **正式合同**：`Unknown` 直接返回错误，业务侧重试 / 提示 UI。
⚠️ **ASR 现阶段 AT 指令集复用 UniSoc**：同一 Quectel 厂家 AT 共通；后端 `ModemFactory` 对 `ChipsetVendor::Asr` 走 `QuectelModem::unisoc(model)`，UI 平台徽标显示 "ASR"。后续按实测逐条调整。
⚠️ **关键字冲突**：`RM500Q`（Qualcomm） vs `RM500U`（UniSoc）靠末尾字母区分；若未来加 `RG500UA` 之类型号需补测试。

---

## 1. 基础信息查询（通用）

| 功能 | AT 指令 | 解析函数 | 备注 |
|------|---------|----------|------|
| SIM 状态 | `AT+CPIN?` | `parse_cpin()` | |
| IMEI | `AT+CGSN` | `parse_cgsn()` | |
| ICCID（UniSoc） | `AT+CCID` | `parse_iccid()` | UniSoc 首选 |
| ICCID（Qualcomm） | `AT+ICCID` | `parse_iccid()` | Qualcomm 首选 |
| 型号 | `AT+CGMM` | `parse_cgmm()` | 用于厂商检测 |
| 厂商 | `AT+CGMI` | `parse_cgmm()` | |
| 固件版本 | `AT+GMR` | `parse_gmr()` | |
| AP/CP 基线 | `AT+QBASELINE` | `parse_qbaseline()` | |
| 温度 | `AT+QTEMP` | `parse_qtemp()` | SOC 温度 / PA 温度 |

## 2. 网络状态查询（通用）

| 功能 | AT 指令 | 解析函数 | 备注 |
|------|---------|----------|------|
| 服务小区 | `AT+QENG="servingcell"` | `parse_qeng_serving_cell()` | 带宽字段 Qualcomm/UniSoc 格式不同 |
| 邻区列表 | `AT+QENG="neighbourcell"` | `parse_qeng_neighbour_cells()` | |
| 运营商 | `AT+COPS?` | `parse_cops_with_act()` | |
| 注册状态 | `AT+CEREG?` | `parse_cereg()` | |
| PDP 激活状态 | `AT+CGACT?` | `parse_cgact()` | |
| 网络模式 | `AT+QNWPREFCFG="mode_pref"` | `parse_qnwprefcfg_mode()` | |

## 3. IP 与数据连接（平台差异）

### UniSoc（RM500U / RG200U / RG500U）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 连接数据 | `AT+QNETDEVCTL={cid},1,1` | `<cid>,<op=1 拨号>,<state=1 自动重连>`（手册 p.194，cid 在首位） |
| 断开数据 | `AT+QNETDEVCTL={cid},0` | `<cid>,<op=0 断开>`；`<state>` 仅 op=1/3 有效 |
| 查询 IP | `AT+QNETDEVSTATUS={cid}` | 响应格式见下方说明 |
| 流量统计 | `AT+QGDCNT?` | `parse_qgdcnt()` |
| 重置流量 | `AT+QGDCNT=0` | |
| 天线信号 | `AT+QANTRSSI?` | `parse_qantrssi()` → `[ANT0, ANT1, ANT2, ANT3]` |

`AT+QNETDEVSTATUS=<cid>` 响应格式（行首带 `+QNETDEVSTATUS:` 前缀，IP 为点分十进制，手册 p.195）：
```
+QNETDEVSTATUS: <ipv4>,<mask>,<gw>,,<dns1>,<dns2>,<ipv6>,,,,<v6dns1>,<v6dns2>
                 [0]   [1]   [2] [3] [4]   [5]   [6]  [7][8][9] [10]    [11]
```

### Qualcomm（RM500Q / RG500Q / RM520N / RG520N / RM551E / RM530F 等）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 连接数据 | `AT+QMAP="connect",{rule},1` | `<rule_num>,<connect=1>`（手册 §12.10） |
| 断开数据 | `AT+QMAP="connect",{rule},0` | `<rule_num>,<connect=0>`；QMAP 无 `"disconnect"` 子命令 |
| 查询 IP | `AT+QMAP="WWAN"` | Qualcomm 当前唯一 live 主路径 |
| 流量统计 | `AT+QGDNRCNT?` | `parse_qgdnrcnt()` |
| 重置流量 | `AT+QGDNRCNT=0` | |
| 天线信号 | `AT+QRSRP` | `parse_qrsrp()` → `[ANT0, ANT1, ANT2, ANT3]` |

### Qualcomm IPPT / 桥接 与 5GLAN-L2（仅 Qualcomm 平台；代码在用，此前文档缺失）

| 功能 | AT 指令 | 解析 / 备注 |
|------|---------|------------|
| 查询 MPDN 规则(IPPT 模式) | `AT+QMAP="MPDN_rule"` | `parse_mpdn_ippt_mode()` |
| 清除规则 0 | `AT+QMAP="mPDN_rule",0` | |
| IPPT 路由 | `AT+QMAP="mPDN_rule",0,1,0,0,1,"FF:FF:FF:FF:FF:FF"` | `<rule>,<profileID>,<VLAN>,<IPPT_mode=0 路由>,<auto_connect>,<MAC>` |
| IPPT 桥接 | `AT+QMAP="mPDN_rule",0,1,0,1,1,"FF:FF:FF:FF:FF:FF"` | `<IPPT_mode=1>` 为桥接 |
| 查询连接状态 | `AT+QMAP="MPDN_status"` | `parse_mpdn_connect_status()` |
| 自动连接 查/设 | `AT+QMAP="auto_connect"[,<rule>,<0/1>]` | `parse_auto_connect()` / `set_auto_connect()` |
| ETH PDU 查/启用 | `AT+QMAP="ETH_PDU"` / `AT+QMAP="ETH_PDU","enable"` | `parse_eth_pdu_enabled()` |
| 5GLAN 查/设 | `AT+QCFG="5glan"` / `AT+QCFG="5glan",{cid},{0/1},{vlan}` | |
| 以太网配置 | `AT+QNWCFG="eth_cfg",{profileID},{eth_mode}` | |
| WDS Profile | `AT+QWDSCFG="profile",{cid},"Ethernet","{apn}",{vlan_start},{vlan_end}` | |

## 4. 频段配置（通用）

| 功能 | AT 指令 | 解析函数 |
|------|---------|----------|
| 支持频段(RF) | `AT+QNWPREFCFG="rf_band"` | `parse_qnwprefcfg_rf_band()` |
| LTE 锁定频段 | `AT+QNWPREFCFG="lte_band"` | `parse_qnwprefcfg_bands()` |
| NR 锁定频段 | `AT+QNWPREFCFG="nr5g_band"` | `parse_qnwprefcfg_bands()` |
| 设置 LTE 频段 | `AT+QNWPREFCFG="lte_band",1:3:5` | 频段号纯数字、冒号分隔、不带引号/不带 B 前缀（手册 §5.24.2，例 `=...,1:2`） |
| 设置 NR 频段 | `AT+QNWPREFCFG="nr5g_band",78:79` | 同上，不带 n 前缀 |
| 重置频段 | `AT+QNWPREFCFG="all_band_reset"` | |

## 5. QoS 信息（通用）

| 功能 | AT 指令 | 解析函数 |
|------|---------|----------|
| QoS 查询 | `AT+C5GQOSRDP={cid}` | `parse_c5gqosrdp()` → `(cqi, ul_bw, dl_bw)` |

## 6. 功能开关（通用）

| 功能 | 查询指令 | 设置指令 | 备注 |
|------|---------|---------|------|
| PCIe 模式 | `AT+QCFG="pcie/mode"` | `AT+QCFG="pcie/mode",{0/1}` | |
| 以太网 | `AT+QCFG="ethernet"` | `AT+QCFG="ethernet",{0/1}` | |
| Proxy ARP | `AT+QCFG="proxyarp"` | `AT+QCFG="proxyarp",{0/1}` | |
| UART AT | `AT+QCFG="uartat"` | `AT+QCFG="uartat",{0/1}` | |
| ETH AT | `AT+QCFG="eth_at"` | `AT+QCFG="eth_at",{0/1}` | 仅 Qualcomm；UniSoc 硬编码 false |
| ADB | `AT+QCFG="usbcfg"` | `AT+QCFG="usbcfg",{...}` | 修改倒数第二个字段 |
| NAPT | `AT+QCFG="napt"` | `AT+QCFG="napt",{0/1}` | |
| Netmask | `AT+QCFG="netmask"` | `AT+QCFG="netmask",{0/1}` | |
| USB 网卡模式 | `AT+QCFG="usbnet"` | `AT+QCFG="usbnet",{mode}` | |
| IMS / VoLTE | `AT+QCFG="ims"` | Qualcomm: `AT+QCFG="ims",<mode>,<enable>` (开=`,1,1` 关=`,2,0`)；UniSoc: `AT+QCFG="ims",<enable>` | 仅前端快捷 AT 直发（`send_raw_at`）；HAL 命令层未实现专用 IPC |

## 7. 高通专用配置（仅 Qualcomm 平台）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 数据接口 | `AT+QCFG="data_interface"` | |
| USB 速度 | `AT+QCFG="usbspeed"` | |
| 以太网驱动 | `AT+QETH="eth_driver"` | |

## 7.6 QMAP 扩展指令（仅 Qualcomm 平台）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| VLAN 查询 | `AT+QMAP="VLAN"` | 返回已启用的 VLAN ID 列表 |
| VLAN 启用 | `AT+QMAP="VLAN",<vid>,"enable",1` | 1=ETH 类型 |
| VLAN 禁用 | `AT+QMAP="VLAN",<vid>,"disable"` | |
| LAN IP 查询 | `AT+QMAP="LANIP"` | 前端快捷 AT，HAL 命令层未实现 |
| LAN IP 设置 | `AT+QMAP="LANIP",<ip>,<mask>` | 前端快捷 AT |
| DMZ 查询 (Qualcomm) | `AT+QMAP="DMZ"` | 前端快捷 AT，HAL 命令层未实现 |
| DMZ 设置 (Qualcomm) | `AT+QMAP="DMZ",<ip>` | 前端快捷 AT |
| DMZ 查询 (UniSoc) | `AT+QDMZ?` | 前端快捷 AT |
| DMZ 设置 (UniSoc) | `AT+QDMZ=<ip>` | 前端快捷 AT |
| MTU 查询 | `AT+QCFG="mtu"` | 前端快捷 AT，HAL 命令层未实现 |
| MTU 设置 | `AT+QCFG="mtu",<value>` | 前端快捷 AT |

---

## 8. 控制命令（通用）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 射频开启 | `AT+CFUN=1` | |
| 射频关闭 | `AT+CFUN=0` | |
| 查询射频状态 | `AT+CFUN?` | 响应 `+CFUN: N` |
| 重启模组 | `AT+CFUN=1,1` | |
| 恢复出厂 | `AT&F`（Qualcomm）/ `AT+QPRTPARA=3`（UniSoc） | 代码按芯片分支下发（`factory_reset()`），非 `AT+QFACT` |
| SIM 卡槽查询 | `AT+QUIMSLOT?` | |
| SIM 卡槽切换 | `AT+QUIMSLOT={1/2}` | |
| LAN IP 查询 | `AT+QCFG="lanip_ex"` | ⚠️ 仅前端快捷 AT，HAL 命令层未实现 |
| LAN IP 设置 | `AT+QCFG="lanip_ex","<gw>","<start>","<end>"` | ⚠️ 仅前端快捷 AT，HAL 命令层未实现 |
| 设置 APN | `AT+QICSGP={cid},{type},"<apn>","<user>","<pass>",{auth}` | |
| 查询所有 APN | `AT+QICSGP?` | `query_apn_list()` 首选 |
| 删除 APN | `AT+CGDCONT={cid}` | |
| 查询所有 PDP | `AT+CGDCONT?` | `query_apn_list()` QICSGP 为空时 fallback |
| 激活/停用 PDP | `AT+CGACT={0/1},{cid}` | |
| 5GLAN 查询 | `AT+QCFG="5glan"` | |
| 5GLAN 设置 | `AT+QCFG="5glan",{cid},{0/1},1` | |
| 小区锁定 | `AT+QNWLOCK="common/5g",1,{arfcn},{pci}` | |
| 频点锁定 | `AT+QNWLOCKFREQ="common/5g",1,{arfcn}` | |
| 清除锁定 | `AT+QNWLOCK="common/5g",0` | |
| PLMN 锁定 | `AT+QSIMLOCK="PN","{password}",2,"{plmn}"` | |
| PLMN 解锁 | `AT+QSIMLOCK="PN","{password}"` | |

## 9. 解析辅助函数

| 函数 | 功能 |
|------|------|
| `is_ok()` | 响应是否以 OK 结尾 |
| `extract_data_lines()` | 提取数据行（过滤回显/OK/ERROR） |
| `parse_qcfg_int()` | 通用 QCFG 整数字段提取 |
| `format_rsrp()` | RSRP 格式化（如 `-94 dBm`） |
| `format_rsrq()` | RSRQ 格式化（如 `-4 dB`） |
| `format_bw()` | 带宽格式化（如 `100 MHz`） |
