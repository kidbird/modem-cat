# AT 指令功能映射

平台划分：
- **UniSoc（展锐）**：RM500U / RG200U / RG500U
- **Qualcomm（高通）**：RM500Q / RG500Q / RM520N / RG520N / RM551E / RM530F / RM530N / RG530F / RG525F / RM501Q

---

## 1. 基础信息查询（通用）

| 功能 | AT 指令 | 解析函数 | 备注 |
|------|---------|----------|------|
| SIM 状态 | `AT+CPIN?` | `parse_cpin()` | |
| IMEI | `AT+CGSN` | `parse_cgsn()` | |
| ICCID（UniSoc） | `AT+CCID` | `parse_iccid()` | UniSoc 首选 |
| ICCID（Qualcomm） | `AT+ICCID` | `parse_iccid()` | Qualcomm 首选 |
| ICCID（备用） | `AT+QCCID` | `parse_iccid()` | 两平台均可 fallback |
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
| 连接数据 | `AT+QNETDEVCTL={cid},3,1` | 格式 `<cid>,<op>,<state>`；`op=3` 激活，`state=1` 启用 URC |
| 断开数据 | `AT+QNETDEVCTL={cid},2,0` | `op=2` 去激活，`state=0` |
| 查询 IP | `AT+QNETDEVSTATUS={cid}` | 响应格式见下方说明 |
| 流量统计 | `AT+QGDCNT?` | `parse_qgdcnt()` |
| 重置流量 | `AT+QGDCNT=0` | |
| 天线信号 | `AT+QANTRSSI?` | `parse_qantrssi()` → `[ANT0, ANT1, ANT2, ANT3]` |

`AT+QNETDEVSTATUS` 响应格式（无 status 前缀字段）：
```
<ipv4>,<mask>,<gw>,,<dns1>,<dns2>,<ipv6>,,,,<v6dns1>,<v6dns2>
  [0]   [1]   [2] [3] [4]   [5]   [6]  [7][8][9] [10]    [11]
```

### Qualcomm（RM500Q / RG500Q / RM520N / RG520N / RM551E / RM530F 等）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 连接数据 | `AT+QMAP="connect",{cid}` | |
| 断开数据 | `AT+QMAP="disconnect",{cid}` | |
| 查询 IP | `AT+QMAP="WWAN"` | fallback: `AT+CGPADDR` |
| 流量统计 | `AT+QGDNRCNT?` | `parse_qgdnrcnt()` |
| 重置流量 | `AT+QGDNRCNT=0` | |
| 天线信号 | `AT+QRSRP` | `parse_qrsrp()` → `[ANT0, ANT1, ANT2, ANT3]` |

## 4. 频段配置（通用）

| 功能 | AT 指令 | 解析函数 |
|------|---------|----------|
| 支持频段 | `AT+QNWPREFCFG=?` | `parse_qnwprefcfg_supported()` |
| LTE 锁定频段 | `AT+QNWPREFCFG="lte_band"` | `parse_qnwprefcfg_bands()` |
| NR 锁定频段 | `AT+QNWPREFCFG="nr5g_band"` | `parse_qnwprefcfg_bands()` |
| 设置 LTE 频段 | `AT+QNWPREFCFG="lte_band","B1:B3:B5"` | |
| 设置 NR 频段 | `AT+QNWPREFCFG="nr5g_band","n1:n3:n5"` | |
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
| ETH AT | `AT+QCFG="eth_at"` | `AT+QCFG="eth_at",{0/1}` | |
| ADB | `AT+QCFG="usbcfg"` | `AT+QCFG="usbcfg",{...}` | 修改倒数第二个字段 |
| NAPT | `AT+QCFG="napt"` | `AT+QCFG="napt",{0/1}` | |
| Netmask | `AT+QCFG="netmask"` | `AT+QCFG="netmask",{0/1}` | |
| USB 网卡模式 | `AT+QCFG="usbnet"` | `AT+QCFG="usbnet",{mode}` | |

## 7. 高通专用配置（仅 Qualcomm 平台）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 数据接口 | `AT+QCFG="data_interface"` | |
| USB 速度 | `AT+QCFG="usbspeed"` | |
| 以太网驱动 | `AT+QETH="eth_driver"` | |

## 8. 控制命令（通用）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 射频开启 | `AT+CFUN=1` | |
| 射频关闭 | `AT+CFUN=0` | |
| 查询射频状态 | `AT+CFUN?` | 响应 `+CFUN: N` |
| 重启模组 | `AT+CFUN=1,1` | |
| 恢复出厂 | `AT+QFACT=0` | |
| SIM 卡槽查询 | `AT+QUIMSLOT?` | |
| SIM 卡槽切换 | `AT+QUIMSLOT={1/2}` | |
| LAN IP 查询 | `AT+QCFG="lanip_ex"` | 响应 `+QCFG: "lanip_ex","<gw>","<start>","<end>"` |
| LAN IP 设置 | `AT+QCFG="lanip_ex","<gw>","<start>","<end>"` | |
| 设置 APN | `AT+QICSGP={cid},{type},"<apn>","<user>","<pass>",{auth}` | |
| 删除 APN | `AT+CGDCONT={cid}` | |
| 激活/停用 PDP | `AT+CGACT={0/1},{cid}` | |
| 5GLAN 查询（UniSoc） | `AT+QCFG="5glan"` | 仅 UniSoc 走 QCFG 路径 |
| 5GLAN 设置（UniSoc） | `AT+QCFG="5glan",{cid},{0/1},1` | 高通平台走 L2 ETH_PDU 流程，见 §9 |
| 小区锁定 | `AT+QNWLOCK="common/5g",1,{arfcn},{pci}` | |
| 频点锁定 | `AT+QNWLOCKFREQ="common/5g",1,{arfcn}` | |
| 清除锁定 | `AT+QNWLOCK="common/5g",0` | |
| PLMN 锁定 | `AT+QSIMLOCK="PN","{password}",2,"{plmn}"` | |
| PLMN 解锁 | `AT+QSIMLOCK="PN","{password}"` | |

## 9. 高通 5GLAN（L2 / Ethernet PDU 方案，仅 Qualcomm 平台）

> 来源：`Quectel_RG520N&RG525F&RG5x0F&RM5x0N_Series_5G_LAN_User_Guide_V1.0.0`（2025-10-27）
> 适用：RG520N / RG525F / RG5x0F / RM5x0N，**仅 R05 固件**支持
> 代码：`modem-hal/src/vendors/quectel/mod.rs::configure_qualcomm_5glan / enable_eth_pdu / connect_qualcomm_5glan / query_qualcomm_5glan_status`

### 9.1 L2 与 L3 方案对比

| 项 | L2（以太网 PDU） | L3（IP PDU） |
|----|------------------|-------------|
| PDU 类型 | Ethernet | IPv4 / IPv6 / IPv4v6 |
| 拨号方 | 模组内部拨号（`AT+QMAP="connect"`） | 外部主机拨号（quectel-CM / MBIM） |
| VLAN 支持 | 支持（0–4094，可省略） | **不支持** |
| 主机接入方式 | **只支持** Module + PHY（以太网），不能 USB | 模组支持的任意方式 |
| 主机配置 | 必须发以太网二层帧（不需要 IP 拨号） | 走 IP 拨号 |
| Allowed-NSSAI 示例 | `03.010102` | `03.010103` |
| 关键 AT | `AT+QNWCFG="eth_cfg"` + `AT+QWDSCFG="profile"` + `AT+QMAP="ETH_PDU"` + `AT+QMAP="mpdn_rule"` | 仅 `AT+CGDCONT`，其余由外部拨号工具完成 |

> 注：`AT+CGDCONT` 的 `PDP_Type` **不允许直接配为 Ethernet**，L2 的 Ethernet 属性通过 `AT+QWDSCFG="profile"` 表达。

### 9.2 L2 配置流程（三步 + 一次重启）

```
步骤 1：配置 PDP profile (configure_qualcomm_5glan)
  └─ AT+QNWCFG="eth_cfg",<profile_id>,<eth_mode>
        eth_mode = 1  → 带 VLAN ID 数据流
        eth_mode = 2  → 不带 VLAN ID（vlan_start = 65535 时使用）
  └─ AT+CGDCONT=<cid>,"IPV4V6","<apn>",,,,,,,,,,,,,,1,"<snssai>",
        APN/SNSSAI 之间需要 13 个空字段（S-NSSAIs_ind 在第 17 位）
  └─ AT+QWDSCFG="profile",<cid>,"Ethernet","<apn>",<vlan_start>,<vlan_end>
        不带 VLAN 时两个 VLAN 参数都填 65535

步骤 2：启用 ETH PDU 会话 (enable_eth_pdu)
  └─ AT+QMAP="ETH_PDU","enable"
  └─ ⚠ 必须重启模组（AT+CFUN=1,1）；查询字段在重启前仍返回 disable
  └─ 启用后，IP-based PDU 拨号（AT+QMAP="connect" 走 IP 路径）将不可用

步骤 3：建立 MPDN 规则并拨号 (connect_qualcomm_5glan)
  └─ AT+QMAP="mpdn_rule",<rule_id>,<cid>,0,0,0
  └─ AT+QMAP="connect",<rule_id>,1
```

PDF 示例（带 VLAN，CID=5、profile_id=1、snssai=03.010102）：

```
AT+QNWCFG="eth_cfg",1,1
AT+CGDCONT=5,"IPV4V6","5glan2",,,,,,,,,,,,,,1,"03.010102",
AT+QWDSCFG="profile",5,"Ethernet","5glan2",2,7
AT+QMAP="ETH_PDU","enable"
（重启模组）
AT+QMAP="mpdn_rule",1,5,0,0,0
AT+QMAP="connect",1,1
```

### 9.3 状态查询（query_qualcomm_5glan_status）

| 查询项 | AT 指令 | 解析函数 | 含义 |
|--------|---------|----------|------|
| ETH PDU 是否启用 | `AT+QMAP="ETH_PDU"` | `qualcomm::parse_eth_pdu_enabled` | 匹配 `+QMAP: "ETH_PDU","enable"` 返回 `true` |
| MPDN 规则绑定 CID | `AT+QMAP="mpdn_rule"` | `qualcomm::parse_mpdn_rule_cid(rule_id=1)` | 返回该 rule 绑定的 PDP CID |
| 是否连通 | `AT+QMAP="MPDN_status"` | `qualcomm::parse_mpdn_connect_status_by_rule(rule_id=1)` | 该 rule 是否处于已连接 |

返回结构 `Qualcomm5GlanStatus { eth_pdu_enabled, mpdn_cid, connected }`。

### 9.4 VLAN 子配置（独立于 5GLAN 主流程）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 查询 VLAN 列表 | `AT+QMAP="VLAN"` | `+QMAP: "VLAN",<id>,<state>`；`id=0` 是未打标的基线项，跳过 |
| 启用 VLAN（ETH 型） | `AT+QMAP="VLAN",<vlan_id>,"enable",1` | 第 4 个参数 `1`=ETH，`2`=USB |
| 关闭 VLAN | `AT+QMAP="VLAN",<vlan_id>,"disable"` | |

Windows 主机不支持配置 VLAN ID；Linux 主机用 `vconfig add eth0 <vid>`。

### 9.5 主机侧前置条件（PHY/Ethernet）

```
AT+QCFG="pcie/mode",1       # 启用 PCIe（rtl8125 / RG520N 内置）
AT+QETH="eth_driver","r8125" # 选择 RTL8125 驱动；外接 QPS615 时改为 pcie/mode=3
（重启模组）
```

L2 方案要求主机以**二层以太帧**与模组通信（不要在主机做 IP 拨号），主机网卡静态配置同网段 IP（如 `172.21.1.1/24`、`172.21.1.105/24`），即可在两个 PC 之间通过 5G 核心网（UPF 当虚拟交换机）做单播 / 多播 / 广播。

### 9.6 平台差异速查

| 能力 | Qualcomm | UniSoc | 其他平台 |
|------|----------|--------|---------|
| Ethernet PDU | ✔ | ✘ | ✔ |
| VLAN | ✔ | ✘ | ✔ |
| 拨号方式 | Module + PHY 内部拨号 | — | USB 外拨 / PHY 内拨 |
| AT 配置入口 | ETH_PDU + QWDSCFG | `AT+QCFG="5glan"` | — |
| 理论速率 | DL 3Gbps / UL 1Gbps | — | DL 1Gbps / UL 500Mbps |

## 10. 解析辅助函数

| 函数 | 功能 |
|------|------|
| `is_ok()` | 响应是否以 OK 结尾 |
| `extract_data_lines()` | 提取数据行（过滤回显/OK/ERROR） |
| `parse_qcfg_int()` | 通用 QCFG 整数字段提取 |
| `format_rsrp()` | RSRP 格式化（如 `-94 dBm`） |
| `format_rsrq()` | RSRQ 格式化（如 `-4 dB`） |
| `format_bw()` | 带宽格式化（如 `100 MHz`） |
