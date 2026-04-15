# Quectel RGx00U & RM500U 系列 AT 命令手册

> 来源: Quectel_RGx00U&RM500U系列_AT命令手册_V1.1.pdf
> 生成时间: 2026-04-15

## 1. 概述

本文档涵盖 Quectel RGx00U 和 RM500U 系列 5G 模组的 AT 命令集。模组支持 USB Serial、 Ethernet、 TTL 等接口进行通信。

### 1.1 基本语法

- **命令格式**: 以 `AT` 或 `at` 开头
- **终止符**: `\r\n` (CRLF)
- **响应格式**: 命令后跟响应内容，以 `OK` 或 `ERROR` 结尾

### 1.2 命令格式约定

| 类型 | 格式 | 说明 |
|------|------|------|
| 查询命令 | `AT+<name>?` | 获取当前参数 |
| 设置命令 | `AT+<name>=<value>` | 设置参数值 |
| 测试命令 | `AT+<name>=?` | 查询支持参数范围 |
| 执行命令 | `AT+<name>` | 执行特定操作 |

---

## 2. 通用指令

### 2.1 AT - 测试指令

**功能**: 测试 AT 指令是否正常工作

**命令**:
```
AT
```

**响应**:
```
OK
```

---

### 2.2 ATE - 回显设置

**功能**: 设置命令回显

**命令**:
```
ATE<value>
```

**参数**:
- `0` - 关闭回显
- `1` - 开启回显

**示例**:
```
ATE1
OK
```

---

### 2.3 AT+GMR - 查询固件版本

**功能**: 查询模组固件版本信息

**命令**:
```
AT+GMR
```

**响应**:
```
RG500U_EU_5G_SA01A06V01
OK
```

---

### 2.4 AT+CGMR - 查询固件版本

**功能**: 查询固件版本（与 AT+GMR 等效）

**命令**:
```
AT+CGMR
```

---

## 3. 串口与网络状态查询

### 3.1 AT+CPIN? - SIM 卡状态

**功能**: 查询 SIM 卡状态

**命令**:
```
AT+CPIN?
```

**响应**:
```
+CPIN: READY
OK
```

**参数说明**:
| 状态 | 说明 |
|------|------|
| READY | SIM 卡正常 |
| SIM PIN | 需要 PIN 码 |
| SIM PUK | 需要 PUK 码 |
| NOT INSERTED | 未检测到 SIM 卡 |

---

### 3.2 AT+CSQ - 信号质量

**功能**: 查询信号质量

**命令**:
```
AT+CSQ
```

**响应**:
```
+CSQ: <rssi>,<ber>
OK
```

**参数说明**:
- `<rssi>`: 信号强度 (0-31, 99=未知)
- `<ber>`: 信道误码率 (0-7, 99=未知)

**信号强度对照表**:
| rssi | 信号强度 | 说明 |
|------|----------|------|
| 0 | -115 dBm | 极弱 |
| 1-10 | -110 to -54 dBm | 中等 |
| 11-20 | -53 to -44 dBm | 良好 |
| 21-31 | -43 to -0 dBm | 优秀 |
| 99 | 未知 | 无信号 |

---

### 3.3 AT+CREG? - 网络注册状态

**功能**: 查询网络注册状态

**命令**:
```
AT+CREG?
```

**响应**:
```
+CREG: <n>,<stat>
OK
```

**参数说明**:
- `<n>`: 0=禁用主动上报, 1=启用主动上报
- `<stat>`: 注册状态

**注册状态对照表**:
| stat | 说明 |
|------|------|
| 0 | 未注册 |
| 1 | 已注册 (本地网) |
| 2 | 未注册 (正在搜索) |
| 3 | 注册被拒绝 |
| 4 | 未知 |
| 5 | 已注册 (漫游) |

---

### 3.4 AT+CGREG? - GPRS 注册状态

**功能**: 查询 GPRS 网络注册状态

**命令**:
```
AT+CGREG?
```

**响应**:
```
+CGREG: <n>,<stat>
OK
```

---

### 3.5 AT+CEREG? - EPS 注册状态

**功能**: 查询 EPS 网络注册状态 (5G SA 模式)

**命令**:
```
AT+CEREG?
```

**响应**:
```
+CEREG: <n>,<stat>[,<tac>,<ci>,<acst>]
OK
```

---

### 3.6 AT+QRSRP - 5G 信号强度

**功能**: 查询 5G RSRP 和 RSRQ

**命令**:
```
AT+QRSRP
```

**响应**:
```
+QRSRP: <rsrp>,<rsrq>
OK
```

**参数说明**:
- `<rsrp>`: 参考信号接收功率 (-140 to -44 dBm)
- `<rsrq>`: 参考信号接收质量 (-20 to -3 dB)

---

### 3.7 AT+QRSRQ - 5G RSRQ 查询

**功能**: 查询 5G RSRQ 值

**命令**:
```
AT+QRSRQ
```

**响应**:
```
+QRSRQ: <rsrq>
OK
```

---

### 3.8 AT+QSNR - 5G SNR 查询

**功能**: 查询 5G 信噪比

**命令**:
```
AT+QSNR
```

**响应**:
```
+QSNR: <snr>
OK
```

**参数说明**:
- `<snr>`: 信噪比 (0-100, 值越大越好)

---

### 3.9 AT+QNWINFO - 网络信息

**功能**: 查询当前网络详细信息

**命令**:
```
AT+QNWINFO
```

**响应**:
```
+QNWINFO: <op_mode>,<lac>,<ci>,<freq>
OK
```

**示例**:
```
+QNWINFO: "NR5G",46000,"00100100",630000
OK
```

---

### 3.10 AT+COPS? - 运营商信息

**功能**: 查询当前连接的运营商

**命令**:
```
AT+COPS?
```

**响应**:
```
+COPS: <mode>[,<format>,<oper>[,<AcT>]]
OK
```

**示例**:
```
+COPS: 0,0,"CHINA MOBILE",9
OK
```

**参数说明**:
- `<mode>`: 0=自动, 1=手动, 4=关闭
- `<format>`: 0=长名称, 1=短名称, 2=数字
- `<oper>`: 运营商名称或 MCCMNC
- `<AcT>`: 接入技术 (9=5G NR, 7= LTE)

---

## 4. 模组信息查询

### 4.1 AT+CGSN - 查询 IMEI

**功能**: 查询模组 IMEI 号

**命令**:
```
AT+CGSN
```

**响应**:
```
<imei>
OK
```

**示例**:
```
AT+CGSN
861234567890123
OK
```

---

### 4.2 AT+CCID - 查询 ICCID

**功能**: 查询 SIM 卡 ICCID

**命令**:
```
AT+CCID
```

**响应**:
```
+CCID: <iccid>
OK
```

---

### 4.3 AT+CIMI - 查询 IMSI

**功能**: 查询 SIM 卡 IMSI

**命令**:
```
AT+CIMI
```

**响应**:
```
<imsi>
OK
```

---

### 4.4 AT+CGMI - 查询制造商

**功能**: 查询模组制造商

**命令**:
```
AT+CGMI
```

**响应**:
```
Quectel
OK
```

---

### 4.5 AT+CGMM - 查询型号

**功能**: 查询模组型号

**命令**:
```
AT+CGMM
```

**响应**:
```
RM500U
OK
```

---

### 4.6 AT+CGMR - 查询固件版本

**功能**: 查询固件版本

**命令**:
```
AT+CGMR
```

**响应**:
```
RG500U_EU_5G_SA01A06V01
OK
```

---

## 5. 网络配置

### 5.1 AT+CNMP - 网络模式选择

**功能**: 设置优先网络模式

**命令**:
```
AT+CNMP=<mode>
```

**参数说明**:
| mode | 说明 |
|------|------|
| 2 | 自动优先 |
| 13 | GSM only |
| 14 | WCDMA only |
| 38 | LTE only |
| 63 | NR 5G only (SA) |
| 64 | LTE + NR 5G |

**示例**:
```
AT+CNMP=63
OK
```

---

### 5.2 AT+CNBP - 频段配置

**功能**: 配置网络频段

**命令**:
```
AT+CNBP=<band>
```

**参数说明**:
- `<band>`: 位掩码表示的频段组合

**常用频段**:
| 频段 | 说明 |
|------|------|
| 0x0000000000000001 | n1 |
| 0x0000000000000002 | n2 |
| 0x0000000000000020 | n5 |
| 0x0000000000000040 | n7 |
| 0x0000000000000100 | n20 |
| 0x0000000000000200 | n28 |
| 0x0000000000000400 | n38 |
| 0x0000000000000800 | n40 |
| 0x0000000000001000 | n41 |
| 0x0000000000002000 | n77 |
| 0x0000000000004000 | n78 |
| 0x0000000000008000 | n79 |

---

### 5.3 AT+ICCID - 查询 SIM ICCID

**功能**: 查询 SIM 卡 ICCID

**命令**:
```
AT+ICCID
```

**响应**:
```
+ICCID: <iccid>
OK
```

---

### 5.4 AT+APN - APN 配置

**功能**: 设置或查询 APN

**查询命令**:
```
AT+APN?
```

**设置命令**:
```
AT+APN=<apn_name>[,<auth_type>[,<username>[,<password>]]]
```

**参数说明**:
- `<apn_name>`: APN 名称
- `<auth_type>`: 0=无, 1=PAP, 2=CHAP

**示例**:
```
AT+APN=cmnet
OK
```

---

### 5.5 AT+QICSGP - PDP 上下文配置

**功能**: 配置 PDP 上下文

**命令**:
```
AT+QICSGP=<cid>,<type>,<apn>[,<username>[,<password>[,<authentication>]]]
```

**参数说明**:
- `<cid>`: PDP 上下文 ID (1-16)
- `<type>`: PDP 类型 ("IP", "IPV6", "IPV4V6")
- `<apn>`: APN 名称

**示例**:
```
AT+QICSGP=1,1,"cmnet"
OK
```

---

## 6. PDP 上下文管理

### 6.1 AT+QIACT - 激活 PDP 上下文

**功能**: 激活 PDP 上下文

**命令**:
```
AT+QIACT=<cid>
```

**示例**:
```
AT+QIACT=1
OK
```

---

### 6.2 AT+QIDEACT - 去激活 PDP 上下文

**功能**: 去激活 PDP 上下文

**命令**:
```
AT+QIDEACT=<cid>
```

**示例**:
```
AT+QIDEACT=1
OK
```

---

### 6.3 AT+QIACT? - 查询 PDP 上下文状态

**功能**: 查询所有 PDP 上下文状态

**命令**:
```
AT+QIACT?
```

**响应**:
```
+QIACT: <cid>,<status>,<type>,<ip>
OK
```

---

## 7. 连接与通信

### 7.1 AT+QIOPEN - 打开 Socket

**功能**: 建立 TCP/UDP 连接

**命令**:
```
AT+QIOPEN=<contextID>,<connectID>,<service_type>,<remote_ip>,<remote_port>[,<local_port>[,<access_mode>]]
```

**参数说明**:
- `<contextID>`: PDP 上下文 ID
- `<connectID>`: Socket 连接 ID (0-11)
- `<service_type>`: "TCP" 或 "UDP"
- `<remote_ip>`: 远程 IP 地址
- `<remote_port>`: 远程端口
- `<local_port>`: 本地端口 (可选)
- `<access_mode>`: 0=缓存模式, 1=透明模式

**示例**:
```
AT+QIOPEN=1,0,"TCP","120.78.12.123",8080
OK
```

---

### 7.2 AT+QICLOSE - 关闭 Socket

**功能**: 关闭 Socket 连接

**命令**:
```
AT+QICLOSE=<connectID>
```

**示例**:
```
AT+QICLOSE=0
OK
```

---

### 7.3 AT+QISEND - 发送数据

**功能**: 发送 TCP/UDP 数据

**命令**:
```
AT+QISEND=<connectID>,<length>
```

**响应** (输入数据后):
```
SEND OK
```

---

### 7.4 AT+QIRD - 接收数据

**功能**: 接收 Socket 数据

**命令**:
```
AT+QIRD=<connectID>,<req_length>
```

**响应**:
```
+QIRD: <recv_length>
<data>
OK
```

---

## 8. 状态监控

### 8.1 AT+QSIMSTAT - SIM 卡热插拔状态

**功能**: 查询或设置 SIM 卡热插拔状态

**查询命令**:
```
AT+QSIMSTAT?
```

**设置命令**:
```
AT+QSIMSTAT=<enable>
```

**参数说明**:
- `<enable>`: 0=禁用, 1=启用

---

### 8.2 AT+QIND - 主动上报控制

**功能**: 设置/查询主动上报功能

**命令**:
```
AT+QIND=<flag>[,<mode>]
```

**参数说明**:
- `<flag>`: 0=读取状态, 1=信号强度, 2=网络状态
- `<mode>`: 0=禁用, 1=启用

---

### 8.3 AT+QLOG - 日志级别

**功能**: 设置模组日志级别

**命令**:
```
AT+QLOG=<level>
```

**参数说明**:
- `<level>`: 0=关闭, 1=error, 2=warning, 3=info, 4=debug

---

## 9. 系统操作

### 9.1 AT+QPOWD - 关机

**功能**: 关闭模组

**命令**:
```
AT+QPOWD=<mode>
```

**参数说明**:
- `<mode>`: 0=正常关机, 1=强制关机

**响应**:
```
OK
+QPOWD: 1
```

---

### 9.2 AT+CRESET - 复位模组

**功能**: 复位模组

**命令**:
```
AT+CRESET
```

**响应**:
```
OK
```

---

### 9.3 AT+QCFG - 扩展配置

**功能**: 扩展配置命令

**命令**:
```
AT+QCFG=<item>[,<value>]
```

**常用配置项**:
| item | 说明 |
|------|------|
| "nw/scanmode" | 网络扫描模式 |
| "nw/scanmodepref" | 扫描优先级 |
| "pdp/cid" | PDP CID 映射 |
| "urc/ri/when" | URC 提示方式 |
| "urc/ri/pin" | URC 引脚配置 |

**示例**:
```
AT+QCFG="urc/ri/when","any"
OK
```

---

### 9.4 AT+QFACT - 恢复出厂设置

**功能**: 恢复出厂设置

**命令**:
```
AT+QFACT=<mode>
```

**参数说明**:
- `<mode>`: 0=恢复所有参数, 1=恢复用户数据

---

## 10. 常用命令速查表

### 状态查询
| 命令 | 功能 |
|------|------|
| `AT+CSQ` | 信号质量 |
| `AT+CREG?` | 网络注册状态 |
| `AT+COPS?` | 运营商信息 |
| `AT+CPIN?` | SIM 卡状态 |
| `AT+CGSN` | IMEI |
| `AT+CCID` | ICCID |
| `AT+CGMM` | 模组型号 |
| `AT+CGMR` | 固件版本 |
| `AT+QNWINFO` | 网络详细信息 |
| `AT+QRSRP` | 5G RSRP/RSRQ |

### 网络配置
| 命令 | 功能 |
|------|------|
| `AT+CNMP?` | 查询网络模式 |
| `AT+CNMP=<mode>` | 设置网络模式 |
| `AT+APN?` | 查询 APN |
| `AT+APN=<apn>` | 设置 APN |
| `AT+QIACT?` | 查询 PDP 状态 |
| `AT+QIACT=<cid>` | 激活 PDP |

### 连接操作
| 命令 | 功能 |
|------|------|
| `AT+QIOPEN=...` | 打开 Socket |
| `AT+QICLOSE=<id>` | 关闭 Socket |
| `AT+QISEND=<id>,<len>` | 发送数据 |
| `AT+QIRD=<id>,<len>` | 接收数据 |

### 系统操作
| 命令 | 功能 |
|------|------|
| `AT+QPOWD=0` | 关机 |
| `AT+CRESET` | 复位 |
| `AT+QFACT=0` | 恢复出厂 |
| `AT+QCFG=...` | 扩展配置 |

---

## 11. UI 显示数据映射

### 状态栏
| UI 显示项 | AT 命令 | 解析方式 |
|-----------|---------|----------|
| 信号强度 | `AT+CSQ` | rssi 转换为图标和 dBm |
| 网络类型 | `AT+QNWINFO` | 从 op_mode 解析 (NR5G/LTE) |
| 运营商 | `AT+COPS?` | 从 oper 字段获取 |
| 注册状态 | `AT+CREG?` | stat=1 或 5 表示已注册 |

### 详细信息
| UI 显示项 | AT 命令 | 解析方式 |
|-----------|---------|----------|
| IMEI | `AT+CGSN` | 直接显示 |
| 型号 | `AT+CGMM` | 直接显示 |
| 固件版本 | `AT+CGMR` | 直接显示 |
| ICCID | `AT+CCID` | 直接显示 |
| IMSI | `AT+CIMI` | 直接显示 |
| SIM 状态 | `AT+CPIN?` | 显示 READY 或错误 |

### 网络信息
| UI 显示项 | AT 命令 | 解析方式 |
|-----------|---------|----------|
| RSRP | `AT+QRSRP` | 直接显示 dBm |
| RSRQ | `AT+QRSRQ` | 直接显示 dB |
| SNR | `AT+QSNR` | 直接显示数值 |
| 频段 | `AT+QNWINFO` | 从 freq 字段解析 |
| PCI | (模组特定) | - |
| TAC/LAC | `AT+CREG?` | 从响应获取 |

### 硬件信息
| UI 显示项 | AT 命令 | 解析方式 |
|-----------|---------|----------|
| 温度 | (模组特定) | - |
| 运行时间 | (模组特定) | - |
