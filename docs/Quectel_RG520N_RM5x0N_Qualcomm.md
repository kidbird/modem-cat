RG520N&RG525F&RG5x0F
&RM5x0N Series
AT Commands Manual
5G Module Series
Version: 1.1
Date: 2025-02-20
Status: Released

5G Module Series
At Quectel, our aim is to provide timely and comprehensive services to our customers. If you
require any assistance, please contact our headquarters:
Quectel Wireless Solutions Co., Ltd.
Building 5, Shanghai Business Park Phase III (Area B), No.1016 Tianlin Road, Minhang District,
Shanghai 200233, China
Tel: +86 21 5108 6236
Email: info@quectel.com
Or our local offices. For more information, please visit:
http://www.quectel.com/support/sales.htm.
For technical support, or to report documentation errors, please visit:
http://www.quectel.com/support/technical.htm.
Or email us at: support@quectel.com.
Legal Notices
We offer information as a service to you. The provided information is based on your requirements and
we make every effort to ensure its quality. You agree that you are responsible for using independent
analysis and evaluation in designing intended products, and we provide reference designs for illustrative
purposes only. Before using any hardware, software or service guided by this document, please read this
notice carefully. Even though we employ commercially reasonable efforts to provide the best possible
experience, you hereby acknowledge and agree that this document and related services hereunder are
provided to you on an “as available” basis. We may revise or restate this document from time to time at
our sole discretion without any prior notice to you.
Use and Disclosure Restrictions
License Agreements
Documents and information provided by us shall be kept confidential, unless specific permission is
granted. They shall not be accessed or used for any purpose except as expressly provided herein.
Copyright
Our and third-party products hereunder may contain copyrighted material. Such copyrighted material
shall not be copied, reproduced, distributed, merged, published, translated, or modified without prior
written consent. We and the third party have exclusive rights over copyrighted material. No license shall
be granted or conveyed under any patents, copyrights, trademarks, or service mark rights. To avoid
ambiguities, purchasing in any form cannot be deemed as granting a license other than the normal non-
exclusive, royalty-free license to use the material. We reserve the right to take legal action for
noncompliance with abovementioned requirements, unauthorized use, or other illegal or malicious use of
the material.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 1 / 282

5G Module Series
Trademarks
Except as otherwise set forth herein, nothing in this document shall be construed as conferring any rights
to use any trademark, trade name or name, abbreviation, or counterfeit product thereof owned by
Quectel or any third party in advertising, publicity, or other aspects.
Third-Party Rights
This document may refer to hardware, software and/or documentation owned by one or more third
parties (“third-party materials”). Use of such third-party materials shall be governed by all restrictions and
obligations applicable thereto.
We make no warranty or representation, either express or implied, regarding the third-party materials,
including but not limited to any implied or statutory, warranties of merchantability or fitness for a particular
purpose, quiet enjoyment, system integration, information accuracy, and non-infringement of any third-
party intellectual property rights with regard to the licensed technology or use thereof. Nothing herein
constitutes a representation or warranty by us to either develop, enhance, modify, distribute, market, sell,
offer for sale, or otherwise maintain production of any our products or any other hardware, software,
device, tool, information, or product. We moreover disclaim any and all warranties arising from the
course of dealing or usage of trade.
Privacy Policy
To implement module functionality, certain device data are uploaded to Quectel’s or third-party’s servers,
including carriers, chipset suppliers or customer-designated servers. Quectel, strictly abiding by the
relevant laws and regulations, shall retain, use, disclose or otherwise process relevant data for the
purpose of performing the service only or as permitted by applicable laws. Before data interaction with
third parties, please be informed of their privacy and data security policy.
Disclaimer
a) We acknowledge no liability for any injury or damage arising from the reliance upon the information.
b) We shall bear no liability resulting from any inaccuracies or omissions, or from the use of the
information contained herein.
c) While we have made every effort to ensure that the functions and features under development are
free from errors, it is possible that they could contain errors, inaccuracies, and omissions. Unless
otherwise provided by valid agreement, we make no warranties of any kind, either implied or
express, and exclude all liability for any loss or damage suffered in connection with the use of
features and functions under development, to the maximum extent permitted by law, regardless of
whether such loss or damage may have been foreseeable.
d) We are not responsible for the accessibility, safety, accuracy, availability, legality, or completeness of
information, advertising, commercial offers, products, services, and materials on third-party websites
and third-party resources.
Copyright © Quectel Wireless Solutions Co., Ltd. 2025. All rights reserved.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 2 / 282

5G Module Series
About the Document
Revision History
Version Date Author Description
James YU/
Evan JIN/
Ozzy ANG/
Amos ZHANG/
- 2022-08-12 Shaun DUAN/ Creation of the document
Joseph WANG/
Pacifier WANG/
Wolf GUAN/
Westring LIU
James YU/
Evan JIN/
Ozzy ANG/
Amos ZHANG/
1.0 2024-02-07 Shaun DUAN/ First official release
Joseph WANG/
Pacifier WANG/
Wolf GUAN/
Westring LIU
1. Deleted the EOL product RM521F-GL.
2. Updated the declaration of AT command examples
James YU/ (Chapter 1.9).
Spawn ZHANG/ 3. Updated the return result after the execution of
1.1 2025-02-20 Amos ZHANG/ AT+QCAINFO in EN-DC mode (Chapter 5.21).
Demon YANG/ 4. Added AT+QETH="eth_driver" (Chapter 11.7).
Westring LIU 5. Updated the explanation of the <domain_name>
and domain name in the command example
(Chapter 12.14).
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 3 / 282

5G Module Series
Contents
About the Document .................................................................................................................................. 3
Contents ...................................................................................................................................................... 4
Table Index ................................................................................................................................................ 10
1 Introduction ....................................................................................................................................... 11
1.1. Scope of the Document ............................................................................................................ 11
1.2. Definitions ................................................................................................................................. 11
1.3. AT Command Syntax ............................................................................................................... 12
1.3.1. Basic Syntax.................................................................................................................. 12
1.3.2. S Parameter Syntax ...................................................................................................... 12
1.3.3. Extended Syntax ........................................................................................................... 12
1.3.4. Entering AT Commands ................................................................................................ 13
1.4. AT Command Responses ........................................................................................................ 13
1.5. Supported Character Sets ....................................................................................................... 13
1.6. AT Command Port .................................................................................................................... 14
1.7. URC (Unsolicited Result Code) ............................................................................................... 14
1.8. Module Turn-off Procedure ...................................................................................................... 14
1.9. Declaration of AT Command Examples ................................................................................... 15
2 General Commands .......................................................................................................................... 16
2.1. ATI Request MT Identification ................................................................................................. 16
2.2. AT+GMI Request Manufacturer Identification ......................................................................... 17
2.3. AT+CGMI Request Manufacturer Identification ...................................................................... 17
2.4. AT+GMM Request MT Model Identification ............................................................................ 18
2.5. AT+CGMM Request MT Model Identification ......................................................................... 18
2.6. AT+GMR Request MT Firmware Version Identification .......................................................... 19
2.7. AT+CGMR Request MT Firmware Version Identification ....................................................... 20
2.8. AT+GSN Request IMEI Number ............................................................................................. 20
2.9. AT+CGSN Request IMEI Number .......................................................................................... 21
2.10. AT&F Reset AT Command Settings to Factory Settings ........................................................ 22
2.11. AT&V Display Current AT Command Settings ........................................................................ 22
2.12. AT&W Store Current AT Command Settings to User-defined Profile ..................................... 23
2.13. ATZ Restore All AT Command Settings from User-defined Profile ......................................... 24
2.14. ATQ Set Result Code Presentation Mode .............................................................................. 24
2.15. ATV MT Response Format ..................................................................................................... 25
2.16. ATE Set Command Echo Mode .............................................................................................. 27
2.17. A/ Repeat Previous Command Line ....................................................................................... 27
2.18. ATS3 Set Command Line Termination Character ................................................................... 28
2.19. ATS4 Set Response Formatting Character ............................................................................ 28
2.20. ATS5 Set Command Line Editing Character .......................................................................... 29
2.21. ATX Set CONNECT Result Code Format and Monitor Call Progress ................................... 30
2.22. AT+CFUN Set UE Functionality .............................................................................................. 30
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 4 / 282

5G Module Series
2.23. AT+CMEE Error Message Format .......................................................................................... 32
2.24. AT+CSCS Select TE Character Set ........................................................................................ 33
2.25. AT+QURCCFG Configure URC Indication Option ................................................................. 34
3 Status Control Commands .............................................................................................................. 36
3.1. AT+CPAS ME Activity Status .................................................................................................. 36
3.2. AT+CEER Extended Error Report .......................................................................................... 37
3.3. AT+QCFG Extended Configuration Settings .......................................................................... 38
3.3.1. AT+QCFG="hsdpacat" HSDPA Category Configuration ............................................. 39
3.3.2. AT+QCFG="hsupacat" HSUPA Category Configuration ............................................. 40
3.3.3. AT+QCFG="rrc" RRC Release Version Configuration ................................................ 41
3.3.4. AT+QCFG="pdp/duplicatechk" Establish Multi PDNs With a Single APN ................... 42
3.3.5. AT+QCFG="risignaltype" RI Signal Output Carrier ..................................................... 43
3.3.6. AT+QCFG="data_interface" Set Network Port/Diagnostic Port Communication via
PCIe/USB Interface ..................................................................................................................... 44
3.3.7. AT+QCFG="pcie/mode" Set PCIe RC/EP Mode ......................................................... 45
3.3.8. AT+QCFG="usbspeed" Set USB Speed Mode ........................................................... 46
3.3.9. AT+QCFG="usbnet" Configure NIC Data Call Method ................................................ 47
3.3.10. AT+QCFG="urc/ri/ring" Set RI Behavior for URC RING .............................................. 48
3.3.11. AT+QCFG="urc/ri/smsincoming" Set RI Behavior for Incoming SMS URCs .............. 50
3.3.12. AT+QCFG="urc/ri/other" Set RI Behavior for Other URCs .......................................... 51
3.4. AT+QINDCFG URC Indication Configuration ......................................................................... 52
4 (U)SIM-Related Commands .............................................................................................................. 54
4.1. AT+CIMI Request IMSI ........................................................................................................... 54
4.2. AT+ICCID Get ICCID .............................................................................................................. 55
4.3. AT+CLCK Facility Lock ........................................................................................................... 55
4.4. AT+CPIN Enter PIN ................................................................................................................ 58
4.5. AT+CPWD Change Password ................................................................................................ 60
4.6. AT+CSIM Generic (U)SIM Access .......................................................................................... 62
4.7. AT+CRSM Restricted (U)SIM Access ..................................................................................... 63
4.8. AT+CCHO Open Logical Channel .......................................................................................... 64
4.9. AT+CCHC Close Logical Channel .......................................................................................... 65
4.10. AT+CGLA Generic UICC Logical Channel Access ................................................................. 66
4.11. AT+QPINC Display PIN Remainder Counter.......................................................................... 67
4.12. AT+QINISTAT Query Initialization Status of (U)SIM Card ...................................................... 68
4.13. AT+QSIMDET (U)SIM Card Detection .................................................................................... 69
4.14. AT+QSIMSTAT (U)SIM Card Insertion Status Report ............................................................ 70
4.15. AT+QUIMSLOT Switch (U)SIM Slot ....................................................................................... 72
5 Network Service Commands ........................................................................................................... 74
5.1. AT+COPS Operator Selection ................................................................................................ 74
5.2. AT+CREG Network Registration Status ................................................................................. 76
5.3. AT+CGREG PS Network Registration Status ......................................................................... 78
5.4. AT+CEREG EPS Network Registration Status....................................................................... 79
5.5. AT+C5GREG 5GS Network Registration Status .................................................................... 81
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 5 / 282

5G Module Series
5.6. AT+CGDCONT Define PDP Context ...................................................................................... 83
5.7. AT+C5GNSSAI 5GS NSSAI Setting ....................................................................................... 86
5.8. AT+C5GNSSAIRDP Read 5GS NSSAI Dynamic Parameters ............................................... 87
5.9. AT+CSQ Signal Quality Report ............................................................................................... 90
5.10. AT+QRSRP Report RSRP ...................................................................................................... 91
5.11. AT+QRSRQ Report RSRQ ..................................................................................................... 92
5.12. AT+QSINR Report SINR ......................................................................................................... 94
5.13. AT+CPOL Preferred Operator List .......................................................................................... 95
5.14. AT+COPN Read Operator Names .......................................................................................... 96
5.15. AT+CTZU Automatic Time Zone Update ................................................................................ 97
5.16. AT+CTZR Time Zone Reporting ............................................................................................. 98
5.17. AT+QLTS Obtain Latest Time Synchronized Through Network ........................................... 100
5.18. AT+QNWINFO Query Network Information .......................................................................... 102
5.19. AT+QSPN Query Service Provider Name ............................................................................ 104
5.20. AT+QENG Query Primary Serving Cell and Neighbour Cell Information ............................. 105
5.21. AT+QCAINFO Query Carrier Aggregation Parameters ......................................................... 111
5.22. AT+QNETRC Get the Cause of Network Rejection .............................................................. 114
5.23. AT+QNWCFG Configure and Query Network Parameters ................................................... 119
5.23.1. AT+QNWCFG="lte_cell_id" Read Cell ID Under LTE ................................................ 119
5.23.2. AT+QNWCFG="nr5g_cell_id" Read Cell ID Under 5G SA ........................................ 120
5.23.3. AT+QNWCFG="wcdma_cqi" Read CQI Under WCDMA .......................................... 121
5.23.4. AT+QNWCFG="up/down" Get Average Uplink and Downlink Rates in Delta Time .. 121
5.23.5. AT+QNWCFG="dss_enable" Enable or Disable DSS Function ................................ 122
5.24. AT+QNWPREFCFG Configure Network Searching Preferences ........................................ 123
5.24.1. AT+QNWPREFCFG="gw_band" Set WCDMA Band ................................................ 124
5.24.2. AT+QNWPREFCFG="lte_band" Set LTE Band ......................................................... 125
5.24.3. AT+QNWPREFCFG="nsa_nr5g_band" Set 5G NSA Band ...................................... 127
5.24.4. AT+QNWPREFCFG="nr5g_band" Set 5G SA Band ................................................. 128
5.24.5. AT+QNWPREFCFG="mode_pref" Set Network Search Mode ................................. 130
5.24.6. AT+QNWPREFCFG="srv_domain" Set Service Domain .......................................... 131
5.24.7. AT+QNWPREFCFG="voice_domain" Set Voice Domain.......................................... 132
5.24.8. AT+QNWPREFCFG="roam_pref" Set Roaming Preference .................................... 133
5.24.9. AT+QNWPREFCFG="ue_usage_setting" Set UE Usage Setting ............................. 134
5.24.10. AT+QNWPREFCFG="policy_band" Read Carrier Policy Band ................................ 135
5.24.11. AT+QNWPREFCFG="ue_capability_band" Query UE Band Capability ................... 136
5.24.12. AT+QNWPREFCFG="rat_acq_order" Set RAT Priority............................................. 137
5.24.13. AT+QNWPREFCFG="nr5g_disable_mode" Disable 5G ........................................... 138
6 Call Related Commands ................................................................................................................. 140
6.1. ATA Answer an Incoming Call ............................................................................................... 140
6.2. ATD Originate a Call ............................................................................................................. 141
6.3. ATH Disconnect Existing Call ............................................................................................... 143
6.4. AT+CVHU Control Voice Call Hang Up ................................................................................ 143
6.5. AT+CHUP Hang Up Voice Call ............................................................................................. 144
6.6. ATS0 Set Ring Count Before Automatic Answering ............................................................. 144
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 6 / 282

5G Module Series
6.7. ATS6 Set Pause Before Blind Dialing ................................................................................... 145
6.8. ATS7 Set Waiting Time for Connection Completion ............................................................. 146
6.9. ATS8 Set Waiting Time for Comma Dial Modifier ................................................................. 147
6.10. ATS10 Set Disconnection Delay After Indicating Data Carrier Absence .............................. 147
6.11. AT+CSTA Select Address Type ............................................................................................. 148
6.12. AT+CLCC List Current Call of MT ......................................................................................... 149
6.13. AT+CR Service Reporting Control ........................................................................................ 150
6.14. AT+CRC Set Extended Format of Incoming Call Indication ................................................. 151
6.15. AT+CRLP Select Radio Link Protocol Parameter................................................................. 152
6.16. AT+QECCNUM Configure Emergency Call Number ............................................................ 154
6.17. AT^DSCI Indicate Call Status ............................................................................................... 157
6.18. AT+VTS Generate DTMF Tone ............................................................................................. 159
6.19. AT+VTD Set DTMF Tone Duration ....................................................................................... 160
7 Phonebook Commands .................................................................................................................. 162
7.1. AT+CNUM Get Subscriber Number ...................................................................................... 162
7.2. AT+CPBF Find Phonebook Entry ......................................................................................... 163
7.3. AT+CPBR Read Phonebook Entry ....................................................................................... 164
7.4. AT+CPBS Select Phonebook Memory Storage ................................................................... 165
7.5. AT+CPBW Write Phonebook Entry ....................................................................................... 166
8 Short Message Service Commands .............................................................................................. 168
8.1. AT+CSMS Select Message Service ..................................................................................... 168
8.2. AT+CMGF Set Message Format .......................................................................................... 169
8.3. AT+CSCA Set Service Center Address ................................................................................ 170
8.4. AT+CPMS Select Message Memory Storage ....................................................................... 171
8.5. AT+CMGD Delete Message.................................................................................................. 173
8.6. AT+CMGL Read Message by Status .................................................................................... 174
8.7. AT+CMGR Read Message by Index..................................................................................... 178
8.8. AT+CMGS Send Message .................................................................................................... 182
8.9. AT+CMMS Send Multiple Messages .................................................................................... 183
8.10. AT+CMGW Write Message to Memory Storage ................................................................... 184
8.11. AT+CMSS Send Messages from Memory Storage .............................................................. 186
8.12. AT+CNMA New Message Acknowledgement ....................................................................... 188
8.13. AT+CNMI Set New Message Indication ............................................................................... 190
8.14. AT+CSCB Select Cell Broadcast Message Type ................................................................. 192
8.15. AT+CSDH Show Text Mode Parameter ................................................................................ 193
8.16. AT+CSMP Set Text Mode Parameter ................................................................................... 194
9 Packet Domain Commands ........................................................................................................... 196
9.1. AT+CGATT Attach to or Detach from PS .............................................................................. 196
9.2. AT+CGACT Activate or Deactivate PDP Context ................................................................. 197
9.3. AT+CGDATA Enter Data State .............................................................................................. 199
9.4. AT+CGPADDR Show PDP Addresses ................................................................................. 200
9.5. AT+CGEREP Report Packet Domain Event ......................................................................... 201
9.6. AT+CGSMS Select Service for MO SMS Messages ............................................................ 203
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 7 / 282

5G Module Series
9.7. AT+QGDNRCNT Packet Data Counter (5G Supported) ...................................................... 204
9.8. AT+QAUGDCNT Auto Save Packet Data Counter ............................................................... 206
9.9. AT+QNETDEVSTATUS Query RmNet Device Status ........................................................ 207
10 Supplementary Service Commands ............................................................................................. 209
10.1. AT+CCFC Call Forwarding Number and Conditions Control ............................................... 209
10.2. AT+CCWA Call Waiting Control ............................................................................................. 211
10.3. AT+CHLD Call-Related Supplementary Services ................................................................ 214
10.4. AT+CLIP Present Calling Line Identification ......................................................................... 216
10.5. AT+CLIR Restrict Calling Line Identification ......................................................................... 217
10.6. AT+COLP Present Connected Line Identification................................................................. 218
10.7. AT+CSSN Supplementary Service Notification .................................................................... 220
10.8. AT+CUSD Unstructured Supplementary Service Data ........................................................ 221
11 Hardware-Related Commands ....................................................................................................... 223
11.1. AT+QPOWD Power off ......................................................................................................... 223
11.2. AT+CCLK Clock .................................................................................................................... 224
11.3. AT+CBC Battery Charge ....................................................................................................... 225
11.4. AT+QADC Read ADC Value ................................................................................................. 225
11.5. AT+QSCLK Set Sleep Mode ................................................................................................. 226
11.6. AT+QAGPIO Set Output Level of AP or PMU GPIO ............................................................ 227
11.7. AT+QETH="eth_driver" Select PCIe Ethernet Controller Driver to be Loaded .................... 228
12 QMAP-Related Commands ............................................................................................................. 230
12.1. AT+QMAP Configure QMAP-Related Parameters ............................................................... 230
12.2. AT+QMAP="WWAN" Query IP Address of Default QMAP Data Call ................................... 231
12.3. AT+QMAP="DMZ" Query/Set DMZ of Default QMAP Data Call .......................................... 232
12.4. AT+QMAP="GRE" Query/Set GRE Data Acceleration ......................................................... 233
12.5. AT+QMAP="LAN" Query/Lock Single IP Address for Default LAN Interface ....................... 234
12.6. AT+QMAP="LANIP" Query/Modify DHCP Address Pool of Default LAN Interface .............. 236
12.7. AT+QMAP="VLAN" Query/Set VLAN ................................................................................... 237
12.8. AT+QMAP="MPDN_rule" Query/Modify QMAP Multiple Data Call Rule ............................. 239
12.9. AT+QMAP="IPPT_NAT" Query/Set IPPT NAT Working Mode of QMAP Data Call ............. 243
12.10. AT+QMAP="connect" Initiate/Terminate QMAP Data Call ................................................... 244
12.11. AT+QMAP="auto_connect" Query/Modify Automatic Connection of QMAP Data Call ........ 245
12.12. AT+QMAP="MPDN_status" Query QMAP Multiple Data Call Status................................... 247
12.13. AT+QMAP="SFE" Query/Set SFE Software Acceleration .................................................... 248
12.14. AT+QMAP="domain" Query/Set Gateway Domain Name of LAN/VLAN Interface.............. 249
12.15. AT+QMAP="DHCPV6DNS" Query/Set IPv6 DNS of QMAP Data Call ................................ 250
12.16. AT+QMAP="DHCPV4DNS" Query/Set IPv4 DNS Proxy of QMAP Data Call...................... 251
13 Appendix References ..................................................................................................................... 253
13.1. Terms and Abbreviations ........................................................................................................ 253
13.2. Factory Default Settings Restorable with AT&F ..................................................................... 260
13.3. AT Command Settings Storable with AT&W .......................................................................... 261
13.4. AT Command Settings Storable with ATZ .............................................................................. 262
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 8 / 282

5G Module Series
13.5. Summary of CME ERROR Codes ......................................................................................... 263
13.6. Summary of CMS ERROR Codes ......................................................................................... 265
13.7. Summary of URC ................................................................................................................... 266
13.8. SMS Character Sets Conversions ......................................................................................... 269
13.9. Release Cause Text List of AT+CEER ................................................................................... 275
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 9 / 282

5G Module Series
Table Index
Table 1: Applicable Modules ........................................................................................................................11
Table 2: Type of AT Commands ................................................................................................................. 12
Table 3: AT&V Response............................................................................................................................ 23
Table 4: Numeric Equivalents and Brief Descriptions of ATV0&ATV1 Result Codes ................................ 26
Table 5: Terms and Abbreviations ............................................................................................................ 253
Table 6: Factory Default Settings Restorable with AT&F ......................................................................... 260
Table 7: AT Command Settings Storable with AT&W ............................................................................... 261
Table 8: AT Command Settings Storable with ATZ .................................................................................. 262
Table 9: Summary of General +CME ERROR: <err> Codes ................................................................... 263
Table 10: Summary of General +CMS ERROR: <err> Codes ................................................................. 265
Table 11: Summary of URC ...................................................................................................................... 266
Table 12: SMS Text Input or Output Methods .......................................................................................... 269
Table 13: Input Conversion Table (DCS=GSM 7-bit and AT+CSCS="GSM") ......................................... 270
Table 14: Output Conversion Table (DCS=GSM 7-bit and AT+CSCS="GSM") ....................................... 270
Table 15: GSM Extended Characters (GSM Encode) ............................................................................. 271
Table 16: Input Conversion Table (DCS = GSM 7-bit and AT+CSCS="IRA") .......................................... 272
Table 17: IRA Extended Characters ......................................................................................................... 273
Table 18: Output Conversion Table (DCS = GSM 7-bit and AT+CSCS="IRA") ....................................... 273
Table 19: GSM Extended Characters (ISO-8859-1/Unicode) .................................................................. 274
Table 20: Release Cause Text List of AT+CEER ..................................................................................... 275
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 10 / 282

5G Module Series
1
Introduction
1.1. Scope of the Document
This document presents the AT command set supported by Quectel 5G modules.
Table 1: Applicable Modules
Module Family Module
- RG520N Series
- RG525F-NA
RG520F Series
RG5x0F
RG530F Series
RM520N Series
RM5x0N
RM530N-GL
1.2. Definitions
⚫ <CR> Carriage return character.
⚫ <LF> Line feed character.
⚫ <...> Parameter name. Angle brackets do not appear on the command line.
⚫ [...] Optional parameter of a command or an optional part of TA information response.
Square brackets do not appear on the command line. When an optional parameter is
not given in a command, the new value equals its previous value or the default
settings, unless otherwise specified.
⚫ Underline Default setting of a parameter.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 11 / 282

5G Module Series
1.3. AT Command Syntax
All command lines must start with AT or at and end with <CR>. Information responses and result codes
always start and end with a carriage return character and a line feed character:
<CR><LF><response><CR><LF>. In tables presenting commands and responses throughout this
document, only the commands and responses are presented, and <CR> and <LF> are deliberately
omitted.
AT commands implemented by the modules can be separated into three categories syntactically: “Basic”,
“S Parameter” and “Extended”, as listed below:
1.3.1. Basic Syntax
Basic command format is AT<x><n>, or AT&<x><n>, where <x> is the command, and <n> is/are the
argument(s) of the command. For example, ATE<n> tells the DCE (Data Circuit-terminating Equipment)
whether received characters should be echoed back to the DTE (Data Terminal Equipment) according to
the value of <n>. <n> is optional and a default will be used if it is omitted.
1.3.2. S Parameter Syntax
S Parameter command format is ATS<n>=<m>, where <n> is the index of the S register to be set, and
<m> is the value to be assigned to it.
1.3.3. Extended Syntax
There are several types of extended commands as shown in the following table.
Table 2: Type of AT Commands
Command Type Syntax Description
Test the existence of the corresponding
Test Command AT+<cmd>=? command and return information about the
type, value, or range of its parameter.
Check the current parameter value of the
Read Command AT+<cmd>?
corresponding command.
Write Command AT+<cmd>=<p1>[,<p2>[,<p3>[...]]] Set user-definable parameter value.
Return a specific information parameter or
Execution Command AT+<cmd>
perform a specific action.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 12 / 282

5G Module Series
1.3.4. Entering AT Commands
Multiple commands can be placed on a single line using a semi-colon (;) between commands. In such
cases, only the first command should have AT prefix. Commands can be in upper or lower case.
Spaces should be ignored when you enter AT commands, except in the following cases:
⚫ Within quoted strings, where they are preserved;
⚫ Within an unquoted string or numeric parameter;
⚫ Within an IP address;
⚫ Within the AT command name up to and including a =, ? or =?.
On input, at least a carriage return is required. A newline character is ignored so it is permissible to use
carriage return/line feed pairs on the input.
If no command is entered after the AT token, OK will be returned. If an invalid command is entered,
ERROR will be returned.
Optional parameters, unless explicitly stated, need to be provided up to the last entered parameter.
1.4. AT Command Responses
When the AT command processor has finished processing a line, it will output OK, ERROR or +CME
ERROR: <err> to indicate that it is ready to accept a new command. Solicited information responses are
sent before the final OK, ERROR or +CME ERROR: <err>.
Responses will be in the format of:
<CR><LF>+CMD1:<parameters><CR><LF>
<CR><LF>OK<CR><LF>
Or
<CR><LF><parameters><CR><LF>
<CR><LF>OK<CR><LF>
1.5. Supported Character Sets
The AT command interfaces of the modules default to the GSM character set. The modules support the
following character sets:
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 13 / 282

5G Module Series
⚫ GSM format
⚫ UCS2
⚫ IRA
The character set can be configured and queried via AT+CSCS (3GPP TS 27.007) and it is defined in
3GPP TS 27.005. The character set affects transmission and reception of SMS and SMS Cell Broadcast
Messages, as well as the entry and display of phone book entries text field.
1.6. AT Command Port
The AT command interface of the module includes the main UART port and two USB ports (USB modem
port and USB AT port), all of which support AT command communication and data transfer.
1.7. URC (Unsolicited Result Code)
Unsolicited Result Code (URC) is a report message that is not issued in response to an executed AT
command. URC is automatically issued by the modules in response to a certain event. Typical URC
triggering events include incoming calls (RING), short message reception, high/low voltage alarm,
high/low temperature alarm.
A summary of the URCs can be found in Chapter 13.7.
1.8. Module Turn-off Procedure
The safest and best way to turn off the module is to execute AT+QPOWD. This procedure is performed
by letting the module log off from the network and allowing the software to enter a secure and safe data
state before disconnecting the power supply.
After sending AT+QPOWD, do not enter any other AT commands. When the command is executed
successfully, the module will output POWERED DOWN and then enter the power down mode. To avoid
data loss, it is suggested to wait for 1 s to disconnect the power supply after the URC POWERED
DOWN is outputted. If POWERED DOWN is not received within 65 s, the power supply will be
disconnected automatically.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 14 / 282

5G Module Series
1.9. Declaration of AT Command Examples
The AT command examples in this document are provided to help you learn about the use of the AT
commands introduced herein. The examples, however, should not be taken as Quectel’s
recommendations or suggestions about how to design a program flow or what status to set the module
into. Sometimes multiple examples may be provided for one AT command. However, this does not mean
that there is a correlation among these examples, or that they should be executed in a given sequence.
The URLs, domain names, IP addresses, usernames/accounts, and passwords (if any) in the AT
command examples are provided for illustrative and explanatory purposes only, and they should be
modified to reflect your actual usage and specific needs.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 15 / 282

5G Module Series
2
General Commands
2.1. ATI Request MT Identification
This command returns the MT identification.
ATI Request MT Identification
Execution Command Response
ATI Quectel
<objectID>
Revision: <revision>
OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<objectID> String type. Device type identifier.
<revision> String type. Identification of MT firmware version.
Example
ATI
Quectel
RG520NNA
Revision: RG520NNAAAR01A01M4G
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 16 / 282

5G Module Series
2.2. AT+GMI Request Manufacturer Identification
This command returns the manufacturer identification. It is identical with AT+CGMI in Chapter 2.3.
AT+GMI Request Manufacturer Identification
Test Command Response
AT+GMI=? OK
Execution Command Response
AT+GMI Quectel
OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
2.3. AT+CGMI Request Manufacturer Identification
This command returns the manufacturer identification. It is identical with AT+GMI in Chapter 2.2.
AT+CGMI Request Manufacturer Identification
Test Command Response
AT+CGMI=? OK
Execution Command Response
AT+CGMI Quectel
OK
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 17 / 282

5G Module Series
2.4. AT+GMM Request MT Model Identification
This command returns the MT model identification. It is identical with AT+CGMM in Chapter 2.5.
AT+GMM Request MT Model Identification
Test Command Response
AT+GMM=? OK
Execution Command Response
AT+GMM <objectID>
OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<objectID> String type. Device type identifier.
2.5. AT+CGMM Request MT Model Identification
This command returns the MT model information. It is identical with the AT+GMM in Chapter 2.4.
AT+CGMM Request MT Model Identification
Test Command Response
AT+CGMM=? OK
Execution Command Response
AT+CGMM <objectID>
OK
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 18 / 282

5G Module Series
Parameter
<objectID> String type. Device type identifier.
2.6. AT+GMR Request MT Firmware Version Identification
This command returns the MT firmware version identification. It is identical with AT+CGMR in
Chapter 2.7.
AT+GMR Request MT Firmware Version Identification
Test Command Response
AT+GMR=? OK
Execution Command Response
AT+GMR <revision>
OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<revision> String type. Identification of MT firmware version, including line terminators, which
should not exceed 2048 characters.
Example
AT+GMR
RG520NNAAAR01A01M4G
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 19 / 282

5G Module Series
2.7. AT+CGMR Request MT Firmware Version Identification
This command returns the MT firmware version. It is identical with AT+GMR in Chapter 2.6.
AT+CGMR Request MT Firmware Version Identification
Test Command Response
AT+CGMR=? OK
Execution Command Response
AT+CGMR <revision>
OK
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<revision> String type. Identification of MT firmware version, including line terminators, which
should not exceed 2048 characters.
2.8. AT+GSN Request IMEI Number
This command returns the IMEI (International Mobile Equipment Identity) number of the ME that permits
the user to identify the individual ME device. It is identical with AT+CGSN in Chapter 2.9.
AT+GSN Request IMEI Number
Test Command Response
AT+GSN=? OK
Execution Command Response
AT+GSN <IMEI>
OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 20 / 282

5G Module Series
Parameter
<IMEI> String type. IMEI number of ME.
NOTE
IMEI can be used to identify an ME since it is unique to each ME.
2.9. AT+CGSN Request IMEI Number
This command requests the IMEI (International Mobile Equipment Identity) number of the ME that
permits the user to identify the individual ME device. It is identical with AT+GSN in Chapter 2.8.
AT+CGSN Request IMEI Number
Test Command Response
AT+CGSN=? OK
Execution Command Response
AT+CGSN <IMEI>
OK
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<IMEI> String type. IMEI number of ME.
NOTE
IMEI can be used to identify an ME since it is unique to each ME.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 21 / 282

5G Module Series
2.10. AT&F Reset AT Command Settings to Factory Settings
This command resets AT command settings to factory settings specified by the manufacturer. See
Chapter 13.2 for the factory settings restorable with AT&F.
AT&F Reset AT Command Settings to Factory Settings
Execution Command Response
AT&F[<value>] OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<value> Integer type.
0 Reset all AT command settings to factory settings.
NOTE
Executing AT&F writes data to NVM (Non-Volatile Memory). Please proceed with caution.
2.11. AT&V Display Current AT Command Settings
This command displays the current settings of some AT command parameters, including the single-letter
AT command parameters that are not otherwise readable. See Table 4 for the default command
response before any change.
AT&V Display Current AT Command Settings
Execution Command Response
AT&V OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 22 / 282

5G Module Series
Table 3: AT&V Response
AT&V
&C: 1
&D: 2
&F: 0
&W: 0
E: 1
Q: 0
V: 1
X: 4
Z: 0
S0: 0
S3: 13
S4: 10
S5: 8
S6: 2
S7: 0
S8: 2
S10: 15
OK
2.12. AT&W Store Current AT Command Settings to User-defined Profile
This command stores the current AT command settings to a user-defined profile in non-volatile memory
(See Chapter 13.3). After this command is executed, the AT command settings are automatically
restored from the user-defined profile during power-up or if ATZ is executed.
AT&W Store Current AT Command Settings to User-defined Profile
Execution Command Response
AT&W[<n>] OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 23 / 282

5G Module Series
0 Profile number to store current AT command settings.
NOTE
Executing AT&W[<n>] writes data to NVM. Please proceed with caution.
2.13. ATZ Restore All AT Command Settings from User-defined Profile
This command first resets the AT command settings to their manufacturer defaults, which is similar to
AT&F. Afterwards the AT command settings are restored from the user-defined profile in the non-volatile
memory, if they have been stored with AT&W (See Chapter 13.4).
Any additional AT command on the same command line may be ignored.
ATZ Restore All AT Command Settings From User-defined Profile
Execution Command Response
ATZ[<value>] OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<value> Integer type.
0 Reset to profile number 0.
2.14. ATQ Set Result Code Presentation Mode
This command controls whether the result code is transmitted to TE. Other information text transmitted
as response is not affected.
ATQ Set Result Code Presentation Mode
Execution Command Response
ATQ<n> If <n>=0:
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 24 / 282

5G Module Series
If <n>=1:
(none)
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type. Whether result code is transmitted to TE.
0 Result code is transmitted
1 Result code is suppressed and not transmitted
2.15. ATV MT Response Format
This command determines the contents of the header and trailer transmitted with AT command result
codes and information responses.
The numeric equivalents and brief descriptions of results codes are listed in the following Table 5.
ATV MT Response Format
Execution Command Response
ATV<value> When <value>=0
0
When <value>=1
OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<value> Integer type.
0 Information response: <text><CR><LF>
Short result code format: <numeric code><CR>
1 Information response: <CR><LF><text><CR><LF>
Long result code format: <CR><LF><verbose code><CR><LF>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 25 / 282

                                          5G Module Series
Example
| ATV1    |       |       |     |   //Set <value>=1.  |     |     |     |
| ------- | ----- | ----- | --- | ------------------- | --- | --- | --- |
OK
AT+CSQ
+CSQ: 30,99

OK                       //When <value>=1, the result code is OK.
| ATV0    |       |       |     |   //Set <value>=0.  |     |     |     |
| ------- | ----- | ----- | --- | ------------------- | --- | --- | --- |
0
AT+CSQ
+CSQ: 30,99
0                        //When <value>=0, the result code is 0.

Table 4: Numeric Equivalents and Brief Descriptions of ATV0&ATV1 Result Codes
| ATV1     |     | ATV0  | Description                          |            |               |                     |       |
| -------- | --- | ----- | ------------------------------------ | ---------- | ------------- | ------------------- | ----- |
| OK       |     | 0     | Acknowledge of a command execution.  |            |               |                     |       |
|          |     |       | A  connection                        | has  been  | established.  | DCE  is  switching  | from  |
| CONNECT  |     | 1     |                                      |            |               |                     |       |
command mode to data mode.
RING  2  DCE has detected an incoming call signal from network.
A connection has been terminated or an attempt to establish a
| NO CARRIER  |     | 3   |     |     |     |     |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- |
connection fails.
|     |     |     | Command  | not  recognized  | due  to  | exceeding  command  | line  |
| --- | --- | --- | -------- | ---------------- | -------- | ------------------- | ----- |
ERROR  4  maximum length, invalid parameter value, or other processing
issues.
| NO DIALTONE  |     | 6   | No dial tone detected.           |     |     |     |     |
| ------------ | --- | --- | -------------------------------- | --- | --- | --- | --- |
| BUSY         |     | 7   | Engaged (busy) signal detected.  |     |     |     |     |
@ (Wait for Quiet Answer) dialing modifier was used, but remote
NO ANSWER  8  ringing followed by five seconds of silence was not detected
before connection timer expired (S7).

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                26 / 282

                                          5G Module Series
2.16. ATE  Set Command Echo Mode

This command controls whether TA echoes characters received from TE in AT command mode.
ATE  Set Command Echo Mode
| Execution Command      |     |     | Response                       |
| ---------------------- | --- | --- | ------------------------------ |
| ATE<value>             |     |     | OK                             |
| Maximum Response Time  |     |     | 300 ms                         |
| Characteristics        |     |     | -                              |
| Reference              |     |     | ITU-T Recommendation V.25 ter  |
Parameter
<value>  Integer type. Whether to echo characters received from TE.
|     | 0   |   Echo mode OFF  |     |
| --- | --- | ---------------- | --- |
|     | 1   |   Echo mode ON   |     |

2.17. A/  Repeat Previous Command Line

This command repeats the previous AT command line, and "/" acts as the line termination character.
A/  Repeat Previous Command Line
| Execution Command  |     |     | Response                       |
| ------------------ | --- | --- | ------------------------------ |
| A/                 |     |     | Repeat the previous command    |
| Characteristics    |     |     | -                              |
| Reference          |     |     | ITU-T Recommendation V.25 ter  |
Example
ATI                   //Deliver the MT identification information text.
Quectel
RG520NNA
Revision: RG520NNAAAR01A01M4G

OK
| A/      |       |       |   //Repeat the previous command.  |
| ------- | ----- | ----- | --------------------------------- |
Quectel
RG520NNA
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                27 / 282

5G Module Series
Revision: RG520NNAAAR01A01M4G
OK
2.18. ATS3 Set Command Line Termination Character
This command determines the character that terminates an incoming command line, which is recognized
by TA. It is also generated by the module for result codes and information text, along with character
value set via ATS4.
ATS3 Set Command Line Termination Character
Read Command Response
ATS3? <n>
OK
Write Command Response
ATS3=<n> OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type. Command line termination character. Range: 0–127. Default: 13.
2.19. ATS4 Set Response Formatting Character
This command determines the character generated by TA for result code and information text, along with
the command line termination character set via ATS3.
ATS4 Set Response Formatting Character
Read Command Response
ATS4? <n>
OK
Write Command Response
ATS4=<n> OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 28 / 282

5G Module Series
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type. Response formatting character. Range: 0–127. Default: 10.
2.20. ATS5 Set Command Line Editing Character
This command determines the editing character used by TA to delete the immediately preceding
character from the command line, i.e., the backspace key.
ATS5 Set Command Line Editing Character
Read Command Response
ATS5? <n>
OK
Write Command Response
ATS5=<n> OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type. Response editing character. Range: 0–127. Default: 8.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 29 / 282

5G Module Series
2.21. ATX Set CONNECT Result Code Format and Monitor Call Progress
This command determines whether TA transmits particular result codes to TE. It also controls whether TA
detects a dial tone when initiating a call and an engaged tone (i.e., busy signal).
ATX Set CONNECT Result Code Format and Monitor Call Progress
Execution Command Response
ATX<value> OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<value> Integer type.
0 Only CONNECT is returned, with both dial tone and busy detection disabled.
1 Only CONNECT<text> is returned, with both dial tone and busy detection disabled.
2 CONNECT<text> is returned, with dial tone detection enabled, and busy detection
disabled.
3 CONNECT<text> is returned, with dial tone detection disabled, and busy detection
enabled.
4 CONNECT<text> is returned, with both dial tone and busy detection enabled.
2.22. AT+CFUN Set UE Functionality
This command controls UE functionality level. It can also be used for resetting UE.
AT+CFUN Set UE Functionality
Test Command Response
AT+CFUN=? +CFUN: (list of supported <fun>s),(list of supported <rst>s)
OK
Read Command Response
AT+CFUN? +CFUN: <fun>
OK
Write Command Response
AT+CFUN=<fun>[,<rst>] OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 30 / 282

                                          5G Module Series

If there is any error:
+CME ERROR: <err>
Or
ERROR
| Maximum Response Time  |     |     |     | 15 s, determined by the network.  |     |
| ---------------------- | --- | --- | --- | --------------------------------- | --- |
| Characteristics        |     |     |     | -                                 |     |
| Reference              |     |     |     | 3GPP TS 27.007                    |     |
Parameter
| <fun>  | Integer type. Functionality level.  |     |     |     |     |
| ------ | ----------------------------------- | --- | --- | --- | --- |
0    Minimum functionality
1    Full functionality
4    Disable both transmitting and receiving RF signal
| <rst>  | Integer type. Whether to reset UE.  |     |     |     |     |
| ------ | ----------------------------------- | --- | --- | --- | --- |
0    Do not reset UE before setting it to <fun> power level.
1    Reset UE. The device is fully functional after the reset. This value is available only
    for <fun>=1.
| <err>  | Error code. For more details, see Chapter 13.5.  |     |     |     |     |
| ------ | ------------------------------------------------ | --- | --- | --- | --- |

NOTE
When  the  module  searches  or  registers  the  network,  it  may  write  data  to  NVM  if  executing
AT+CFUN=1. Please proceed with caution.
Example
AT+CFUN=0                   //Switch UE to minimum functionality.
OK
| AT+COPS?  |     |     |     |     |   //Read command.               |
| --------- | --- | --- | --- | --- | ------------------------------- |
| +COPS: 0  |     |     |     |     |   //No operator is registered.  |

OK
AT+CPIN?
| +CME ERROR: 13  |     |     |     |     |   //(U)SIM failure  |
| --------------- | --- | --- | --- | --- | ------------------- |
AT+CFUN=1                   //Switch UE to full functionality.
OK

+CPIN: SIM PIN
| AT+CPIN=1234  |     |     |     |     |   //Enter PIN.  |
| ------------- | --- | --- | --- | --- | --------------- |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                31 / 282

                                          5G Module Series
OK

+CPIN: READY

+QUSIM: 1

+QIND: PB DONE

+QIND: SMS DONE
| AT+CPIN?  |       |       |     //Read command.  |
| --------- | ----- | ----- | -------------------- |
+CPIN: READY

OK
| AT+COPS?  |       |       |     //Read command.  |
| --------- | ----- | ----- | -------------------- |
+COPS: 0,0,"CHINA MOBILE CMCC",7        //Operator is registered.
OK

2.23. AT+CMEE  Error Message Format

This command disables or enables the use of final result code +CME ERROR: <err> as the indication
for errors. When enabled, errors will trigger +CME ERROR: <err> final result code instead of ERROR.
AT+CMEE  Error Message Format
| Test Command  |     |     | Response                         |
| ------------- | --- | --- | -------------------------------- |
| AT+CMEE=?     |     |     | +CMEE: (list of supported <n>s)  |

OK
| Read Command  |     |     | Response    |
| ------------- | --- | --- | ----------- |
| AT+CMEE?      |     |     | +CMEE: <n>  |

OK
| Write Command          |     |     | Response        |
| ---------------------- | --- | --- | --------------- |
| AT+CMEE=[<n>]          |     |     | OK              |
| Maximum Response Time  |     |     | 300 ms          |
| Characteristics        |     |     | -               |
| Reference              |     |     | 3GPP TS 27.007  |

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                32 / 282

                                          5G Module Series
Parameter
| <n>  | Integer type. Whether to enable result code.  |     |     |
| ---- | --------------------------------------------- | --- | --- |
0    Disable result code and use ERROR instead.
1    Enable result code and use numeric values.
2    Enable result code and use verbose values.
| <err>  | Error code. For more details, see Chapter 13.5.  |     |     |
| ------ | ------------------------------------------------ | --- | --- |
Example
| AT+CMEE=0   |     |       |     //Disable result code.  |
| ----------- | --- | ----- | --------------------------- |
OK
| AT+CPIN?  |       |       |     //Read command.              |
| --------- | ----- | ----- | -------------------------------- |
| ERROR     |       |       |     //Only ERROR is displayed.   |
AT+CMEE=1                 //Enable error result code with numeric values.
OK
| AT+CPIN?  |       |       |     //Read command.  |
| --------- | ----- | ----- | -------------------- |
+CME ERROR: 10
AT+CMEE=2                 //Enable error result code with verbose (string) values.
OK
| AT+CPIN?  |       |       |     //Read command.  |
| --------- | ----- | ----- | -------------------- |
+CME ERROR: SIM not inserted

2.24. AT+CSCS  Select TE Character Set

This command informs the module of the character set used by TE. This enables MT to convert
character strings correctly between TE and MT character sets.
AT+CSCS  Select TE Character Set
| Test Command  |     |     | Response                             |
| ------------- | --- | --- | ------------------------------------ |
| AT+CSCS=?     |     |     | +CSCS: (list of supported <chset>s)  |

OK
| Read Command  |     |     | Response        |
| ------------- | --- | --- | --------------- |
| AT+CSCS?      |     |     | +CSCS: <chset>  |

OK
| Write Command    |     |     | Response  |
| ---------------- | --- | --- | --------- |
| AT+CSCS=<chset>  |     |     | OK        |
Or
ERROR
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                33 / 282

5G Module Series
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<chset> String type. Character set.
"GSM" GSM default alphabet
"IRA" International reference alphabet
"UCS2" UCS2 alphabet
Example
AT+CSCS? //Query the current character set.
+CSCS: "GSM" //The character set is GSM.
OK
AT+CSCS="UCS2" //Set the character set to "UCS2".
OK
AT+CSCS? //Query the current character set.
+CSCS: "UCS2" //The character set is UCS2 after the configuration.
OK
2.25. AT+QURCCFG Configure URC Indication Option
This command configures URC output port.
AT+QURCCFG Configure URC Indication Option
Test Command Response
AT+QURCCFG=? +QURCCFG: "urcport",(list of supported <URC_port_value>s)
OK
Write Command Response
AT+QURCCFG="urcport"[,<URC_ If the optional parameter is omitted, query the current setting:
port_value>] +QURCCFG: "urcport",<URC_port_value>
OK
If the optional parameter is specified, set URC output port:
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 34 / 282

5G Module Series
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<URC_port_value> String type. URC output port
"usbat" USB AT port
"usbmodem" USB modem port
"uart1" Main UART
"all" All ports
NOTE
Executing AT+QURCCFG="urcport",<URC_port_value> writes data to NVM. Please proceed with
caution.
Example
AT+QURCCFG=? //Test command.
+QURCCFG: "urcport",("usbat","usbmodem","uart1","all")
OK
AT+QURCCFG="urcport" //Query the current configuration of URC output port.
+QURCCFG: "urcport","usbat"
OK
AT+QURCCFG="urcport","usbmodem" //Configure the URC output port to USB modem port.
OK
AT+QURCCFG="urcport" //Query the current configuration of URC output port.
+QURCCFG: "urcport","usbmodem"
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 35 / 282

5G Module Series
3
Status Control Commands
3.1. AT+CPAS ME Activity Status
This command queries the activity status of ME.
AT+CPAS ME Activity Status
Test Command Response
AT+CPAS=? +CPAS: (list of supported <pas>s)
OK
Execution Command Response
AT+CPAS TA returns the activity status of ME:
+CPAS: <pas>
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<pas> Integer type. ME activity status.
0 Ready
3 Ringing
4 Call in progress or call on hold
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 36 / 282

                                          5G Module Series
Example
| AT+CPAS   |       |       |   //Execution command.  |
| --------- | ----- | ----- | ----------------------- |
| +CPAS: 0  |       |       |   //ME is ready.        |

OK
RING
| AT+CLCC  |       |       |   //Execution command.  |
| -------- | ----- | ----- | ----------------------- |
+CLCC: 1,1,4,0,0,"15695519173",161

OK
| AT+CPAS   |       |       |   //Execution command.  |
| --------- | ----- | ----- | ----------------------- |
| +CPAS: 3  |       |       |   //MT is ringing.      |

OK
| AT+CLCC  |       |       |   //Execution command.  |
| -------- | ----- | ----- | ----------------------- |
+CLCC: 1,0,0,0,0,"10010",129

OK
| AT+CPAS            |       |       |   //Execution command.  |
| ------------------ | ----- | ----- | ----------------------- |
| +CPAS: 4           |       |       |   //Call in progress.   |

OK

3.2.  AT+CEER  Extended Error Report

This command queries an extended error and reports the cause of the last failed operation, such as:
⚫  Failure to release a call
⚫  Failure to set up a call (both mobile originated or terminated)
⚫  Failure to modify a call by using supplementary services
⚫  Failure to activate/deactivate, register/ deregister, or query a supplementary service

The release cause <text> is a text that describes the cause information provided by the network.
AT+CEER  Extended Error Report
| Test Command       |     |     | Response       |
| ------------------ | --- | --- | -------------- |
| AT+CEER=?          |     |     | OK             |
| Execution Command  |     |     | Response       |
| AT+CEER            |     |     | +CEER: <text>  |

OK

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                37 / 282

5G Module Series
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<text> Release cause text. Reason for the last call-related failure, see Chapter 13.9 for details. Both
CS and PS domain call types are reported. Cause data is captured from call manager events
and cached locally for later use by this command.
<err> Error code. For more details, see Chapter 13.5.
3.3. AT+QCFG Extended Configuration Settings
This command queries and configures various settings of UE.
AT+QCFG Extended Configuration Settings
Test Command Response
AT+QCFG=? +QCFG: "hsdpacat",(list of supported <cat>s)
+QCFG: "hsupacat",(list of supported <cat>s)
+QCFG: "rrc",(list of supported <rrcr>s)
+QCFG: "pdp/duplicatechk",(list of supported <enable>s)
+QCFG: "risignaltype",(list of supported <risignatype>s)
+QCFG: "data_interface",(list of supported <network>s),(list of sup
ported <diag>s)
+QCFG: "pcie/mode",(list of supported <mode>s)
+QCFG: "usbnet",(list of supported <net>s)
+QCFG: "usbspeed",(list of supported <speed>s)
+QCFG: "urc/ri/ring",(list of supported <typeri>s),(list of supported
<pulse_duration>s),(list of supported <active_duration>s), (list of
supported <inactive_duration>s),(list of supported
<ring_no_disturbing>s),(list of supported <pulse_count>s)
+QCFG: "urc/ri/smsincoming",(list of supported <typeri>s),(list of
supported <pulse_duration>s),(list of supported <pulse_count>s)
+QCFG: "urc/ri/other",(list of supported <typeri>s),(list of
supported <pulse_duration>s),(list of supported <pulse_count>s)
…
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 38 / 282

5G Module Series
OK
Maximum Response Time 300 ms
3.3.1. AT+QCFG="hsdpacat" HSDPA Category Configuration
This command specifies the HSDPA category.
AT+QCFG="hsdpacat" HSDPA Category Configuration
Write Command Response
AT+QCFG="hsdpacat"[,<cat>] If the optional parameter is omitted, query the current setting:
+QCFG: "hsdpacat",<cat>
OK
If the optional parameter is specified, set the HSDPA category:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<cat> Integer type. HSDPA category.
6 Category 6
8 Category 8
10 Category 10
12 Category 12
14 Category 14
18 Category 18
20 Category 20
24 Category 24
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 39 / 282

5G Module Series
NOTE
Executing AT+QCFG="hsdpacat",<cat> writes data to NVM. Please proceed with caution.
3.3.2. AT+QCFG="hsupacat" HSUPA Category Configuration
This command specifies the HSUPA category.
AT+QCFG="hsupacat" HSUPA Category Configuration
Write Command Response
AT+QCFG="hsupacat"[,<cat>] If the optional parameter is omitted, query the current configuration:
+QCFG: "hsupacat",<cat>
OK
If the optional parameter is specified, set the HSUPA category:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<cat> Integer type. HSUPA category.
5 Category 5
6 Category 6
7 Category 7
8 Category 8
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+QCFG="hsupacat",<cat> writes data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 40 / 282

5G Module Series
3.3.3. AT+QCFG="rrc" RRC Release Version Configuration
This command specifies the RRC release version.
AT+QCFG="rrc" RRC Release Version Configuration
Write Command Response
AT+QCFG="rrc"[,<rrcr>] If the optional parameter is omitted, query the current setting:
+QCFG: "rrc",<rrcr>
OK
If the optional parameter is specified, set the RRC release version:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<rrcr> Integer type. RRC release version.
0 R99
1 R5
2 R6
3 R7
4 R8
5 R9
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+QCFG="rrc",<rrcr> writes data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 41 / 282

5G Module Series
3.3.4. AT+QCFG="pdp/duplicatechk" Establish Multi PDNs With a Single APN
This command allows or refuses establishing multi PDNs with a single APN profile.
AT+QCFG="pdp/duplicatechk" Establish Multi PDNs With a Single APN
Write Command Response
AT+QCFG="pdp/duplicatechk" If the optional parameter is omitted, query the current setting:
[,<enable>] +QCFG: "pdp/duplicatechk",<enable>
OK
If the optional parameter is specified, allow or refuse establishing
multiple PDNs with a single APN profile:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<enable> Integer type.
0 Refuse to establish multi PDNs with a single APN profile
1 Allow to establish multi PDNs with a single APN profile
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+QCFG="pdp/duplicatechk",<enable> writes data to NVM. Please proceed with
caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 42 / 282

5G Module Series
3.3.5. AT+QCFG="risignaltype" RI Signal Output Carrier
This command specifies the RI (ring indicator) signal output carrier.
AT+QCFG="risignaltype" RI Signal Output Carrier
Write Command Response
AT+QCFG="risignaltype"[,<r If the optional parameter is omitted, query the current setting:
isignatype>] +QCFG: "risignaltype",<risignatype>
OK
If the optional parameter is specified, set the RI signal output carrier:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<risignaltype> String type. RI signal output carrier.
"respective" The ring indicator behaves according to the port where URC is
presented. If URC is presented on UART port, it acts as a physical
ring indicator. If on USB port, it acts as a virtual ring indicator. If on
USB AT port which does not support a ring indicator, then there is
no ring indicator. Use AT+QURCCFG="urcport" to determine the
port on which URC is presented, see Chapter 2.25.
"physical" Regardless of the port where URC is presented, it only affects the
behavior of physical ring indicator.
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+QCFG="risignaltype",<risignatype> writes data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 43 / 282

5G Module Series
3.3.6. AT+QCFG="data_interface" Set Network Port/Diagnostic Port
Communication via PCIe/USB Interface
This command sets the network port/diagnostic port communication via USB/PCIe interface.
AT+QCFG="data_interface" Set Network Port/Diagnostic Port Communication via
PCIe/USB Interface
Write Command Response
AT+QCFG="data_interface"[ If the optional parameters are omitted, query the current setting:
,<network>,<diag>] +QCFG: "data_interface",<network>,<diag>
OK
If the optional parameters are specified, set the network
port/diagnostic port communication via USB/PCIe interface:
OK
If there is any error:
ERROR
Maximum Response Time
300 ms
The command takes effect after the module is rebooted.
Characteristics
The configurations are saved automatically.
Parameter
<network> Integer type.
0 Set the network port communication via USB interface.
1 Set the network port communication via PCIe interface. See note 3.
<diag> Integer type.
0 Set the diagnostic port communication via USB interface.
NOTE
1. If the network port and diagnostic port communication is switched to PCIe through eFuse, this
command is invalid, and the communication cannot be switched back to USB.
2. If the network port is set to communicate via the USB interface, the PCIe interface is disabled, i.e.,
no AT port or diagnostic port communicates via the PCIe interface.
3. PCIe switching with AT+QCFG="data_interface" is only possible when the host has an ARM
system and the USB interface of the module is connected to the host.
4. PCIe switching through eFuse supports firmware upgrading when the host is connected via the
PCIe interface. However, PCIe switching via AT+QCFG="data_interface" does not support PCIe-
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 44 / 282

5G Module Series
based upgrading, ensure to use the USB interface for firmware upgrading.
5. When rebooting the module (For example: 5 seconds after upgrading firmware via FOTA or host
connection), ensure to synchronously reboot both the host and module while maintaining the same
power-on time sequence as during the first initialization.
6. It is not recommended to execute AT+CFUN=1,1 to restart the module with the PCIe interface, as
it may cause the PCIe initialization time sequence errors, resulting in PCIe interface initialization
failure. It is recommended to restart the module by hardware method instead.
7. If the module or the host restarts, ensure that the PCIe interface initializes correctly in the proper
time sequence.
Example
AT+QCFG="data_interface" //Query the current configuration.
+QCFG: "data_interface",0,0
OK
AT+QCFG="data_interface",1,0 //Set the network port communication via PCIe interface, and
diagnostic port communication via USB interface, enabling AT
commands to communicate via both interfaces.
OK
3.3.7. AT+QCFG="pcie/mode" Set PCIe RC/EP Mode
This command sets PCIe RC/EP mode.
AT+QCFG="pcie/mode" Set PCIe RC/EP Mode
Write Command Response
AT+QCFG="pcie/mode"[,<mod If the optional parameter is omitted, query the current setting:
e>] +QCFG: "pcie/mode",<mode>
OK
If the optional parameter is specified, set PCIe RC/EP mode:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 45 / 282

5G Module Series
Parameter
<mode> Integer type. PCIe RC or EP mode.
0 PCIe EP mode.
1 PCIe RC mode.
Example
AT+QCFG="pcie/mode" //Query the current configuration.
+QCFG: "pcie/mode",0
OK
AT+QCFG="pcie/mode",1 //Set PCIe RC/EP mode to PCIe RC mode.
OK
3.3.8. AT+QCFG="usbspeed" Set USB Speed Mode
This command sets USB speed mode when the device is inserted in a USB 3.0 (USB 3.1 Gen 1/USB 3.1
Gen 2) port.
AT+QCFG="usbspeed" Set USB Speed Mode
Write Command Response
AT+QCFG="usbspeed"[,<speed> If the optional parameter is omitted, query the current setting:
] +QCFG: "usbspeed",<speed>
OK
If the optional parameter is specified, set USB speed mode:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<speed> String type. USB speed mode.
''20'' USB 2.0 high speed, 480 Mbps
''311'' USB 3.1 Gen1, 5 Gbps
''312'' USB 3.1 Gen2, 10 Gbps
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 46 / 282

5G Module Series
Example
AT+QCFG="usbspeed" //Query the current configuration.
+QCFG: "usbspeed","312"
OK
AT+QCFG="usbspeed","20" //Set USB speed mode to USB 2.0 high speed, 480 Mbps.
OK
3.3.9. AT+QCFG="usbnet" Configure NIC Data Call Method
This command configures NIC data call method in USB NIC mode.
AT+QCFG="usbnet" Configure NIC Data Call Method
Test Command Response
AT+QCFG=? …
+QCFG: "usbnet",(list of supported <net>s)
…
OK
Write Command Response
AT+QCFG="usbnet"[,<net>] If the optional parameter is omitted, query the current
configuration:
+QCFG: "usbnet",<net>
OK
If the optional parameter is specified, set the NIC data call method
in USB NIC mode:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
This command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<net> Integer type. NIC data call method in USB NIC mode.
0 RMNET
1 ECM
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 47 / 282

5G Module Series
2 MBIM
3 RNDIS
Example
AT+QCFG="usbnet" //Query the current configuration.
+QCFG: "usbnet",0
OK
AT+QCFG="usbnet",1 //Set the NIC data call method to ECM
OK
3.3.10. AT+QCFG="urc/ri/ring" Set RI Behavior for URC RING
AT+QCFG="urc/ri/ring", AT+QCFG="urc/ri/smsincoming" (Chapter 3.3.11) and
AT+QCFG="urc/ri/other" (Chapter 3.3.12) control the RI (ring indicator) behavior when a URC is
reported. These configurations will be stored into NV automatically.
The ring indicator is active low. AT+QCFG="urc/ri/ring" specifies the RI behavior when URC RING is
reported to indicate an incoming call.
The sum of <active_duration> and <inactive_duration> determines the interval of URC RING for an
incoming call.
AT+QCFG="urc/ri/ring" Set RI Behavior for URC RING
Write Command Response
AT+QCFG="urc/ri/ring"[,<typeri>[,<pul If the optional parameters are omitted, query the current
se_duration>[,<active_duration>[,<ina setting:
ctive_duration>[,<ring_no_disturbing> +QCFG: "urc/ri/ring",<typeri>,<pulse_duration>,<active
[,<pulse_count>]]]]]] _duration>,<inactive_duration>,<ring_no_disturbing>,<
pulse_count>
OK
If any of the optional parameters is specified, set the RI
behavior when RING URC is reported:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 48 / 282

5G Module Series
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Parameter
<typeri> String type. RI behavior when URC RING is reported.
"off" No change. Ring indicator keeps inactive.
"pulse" Pulse. Pulse width is determined by <pulse_duration>.
"always" Change to active.
"auto" When URC RING is presented to indicate an incoming call, the ring
indicator changes to active and remains active. Answering or hanging up
the incoming call changes the ring indicator state to inactive.
"wave" When URC RING is reported to indicate an incoming call, the ring
indicator outputs a square wave. Both <active_duration> and
<inactive_duration> are used for setting the square wave parameters.
Answering or hanging up the incoming call changes the ring indicator to
inactive.
<pulse_duration> Integer type. Pulse width. Range: 1–2000. Default value: 120. Unit: ms. This
parameter is only valid when <typeri> is "pulse". If this parameter is not
needed, it can be set as null.
<active_duration> Integer type. Active duration of square wave. Range: 1–10000. Default
value: 1000. Unit: ms. This parameter is only valid when <typeri> is "wave".
<inactive_duration> Integer type. Inactive duration of square wave. Range: 1–10000. Default
value:5000. Unit: ms. This parameter is only valid when <typeri> is "wave".
<ring_no_disturbing> String type. Whether the ring indicator behavior can be affected. This
parameter is only valid when <typeri> is configured as "auto" or "wave". For
example, when <typeri> is "wave", if the square wave should not be affected
by other URCs (including SMS-related URCs), then <ring_no_disturbing>
should be set to "on".
"off" RI behavior can be affected by other URCs when it is triggered by
an incoming call ringing.
"on" RI behavior cannot be affected by other URCs when it is triggered
by an incoming call ringing.
<pulse_count> Integer type. Pulse count. Range: 1–5. Default value: 1. This parameter is
only valid when <typeri> is "pulse". Interval between two pulses equals
<pulse_duration>.
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing
AT+QCFG="urc/ri/ring",<typeri>[,<pulse_duration>[,<active_duration>[,<inactive_duration>[,<rin
g_no_disturbing>[,<pulse_count>]]]]] writes data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 49 / 282

5G Module Series
3.3.11. AT+QCFG="urc/ri/smsincoming" Set RI Behavior for Incoming SMS URCs
This command specifies the RI (ring indicator) behavior when related incoming message URC is
presented. Related incoming message URCs list: +CMTI, +CMT, +CDS and +CBM. See Chapter 13.7.
AT+QCFG="urc/ri/smsincoming" Set RI Behavior for Incoming SMS URCs
Write Command Response
AT+QCFG="urc/ri/smsincoming"[,<typ If the optional parameters are omitted, query the current
eri>[,<pulse_duration>[,<pulse_count setting:
>]]] +QCFG: "urc/ri/smsincoming",<typeri>,<pulse_duratio
n>,<pulse_count>
OK
If any of the optional parameters is specified, set the RI
behavior for incoming SMS URCs:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Parameter
<typeri> String type. RI behavior for incoming SMS URCs.
"off" No change. Ring indicator remains inactive.
"pulse" Pulse. Pulse width is determined by <pulse_duration>.
"always" Change to active.
<pulse_duration> Integer type. Pulse width. Range: 1–2000. Default value: 120. Unit: ms.
This parameter is only valid when <typeri> is "pulse".
<pulse_count> Integer type. Pulse count. Range: 1–5. Default value: 1. This parameter is
only valid when <typeri> is "pulse". Interval between two pulses equals
<pulse_duration>.
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+QCFG="urc/ri/smsincoming",<typeri>[,<pulse_duration>[,<pulse_count>]] writes
data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 50 / 282

5G Module Series
3.3.12. AT+QCFG="urc/ri/other" Set RI Behavior for Other URCs
This command specifies the RI (ring indicator) behavior when other URCs are reported.
AT+QCFG="urc/ri/other" Set RI Behavior for Other URCs
Write Command Response
AT+QCFG="urc/ri/other"[,<typeri>[,<p If the optional parameters are omitted, query the current
ulse_duration>[,<pulse_count>]]] setting:
+QCFG: "urc/ri/other",<typeri>,<pulse_duration>,<pulse
_count>
OK
If any of the optional parameters is specified, set the RI
behavior for other URCs:
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Parameter
<typeri> String type. RI behavior for other URCs.
"off" No change. Ring indicator remains inactive.
"pulse" Pulse. Pulse width is determined by <pulse_duration>.
<pulse_duration> Integer type. Pulse width. Range: 1–2000. Default value: 120. Unit: ms. This
parameter is valid only when <typeri> is "pulse".
<pulse_count> Integer type. Pulse count. Range: 1–5. Default value: 1. This parameter is only
valid when <typeri> is "pulse". Interval between two pulses equals
<pulse_duration>.
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+QCFG="urc/ri/other",<typeri>[,<pulse_duration>[,<pulse_count>]] writes data to
NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 51 / 282

5G Module Series
3.4. AT+QINDCFG URC Indication Configuration
This command controls URC indication.
AT+QINDCFG URC Indication Configuration
Test Command Response
AT+QINDCFG=? +QINDCFG: "all",(list of supported <enable>s),(list of supported
<savetonvram>s)
+QINDCFG: "csq",(list of supported <enable>s),(list of supported
<savetonvram>s)
+QINDCFG: "smsfull",(list of supported <enable>s),(list of supported
<savetonvram>s)
+QINDCFG: "ring",(list of supported <enable>s),(list of supported
<savetonvram>s)
+QINDCFG: "smsincoming",(list of supported <enable>s),(list of
supported <savetonvram>s)
+QINDCFG: "act",(list of supported <enable>s),(list of supported
<savetonvram>s)
OK
Write Command Response
AT+QINDCFG=<URC_type>[, If the optional parameters are omitted, query the current configuration:
<enable>[,<savetonvram>]] +QINDCFG: <URC_type>,<enable>
OK
If any of the optional parameters is specified, set the URC indication
configurations:
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
Whether to save configuration depends on <savetonvram>.
Parameter
<URC_type> String type. URC type.
"all" URC master switch. Default: ON.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 52 / 282

5G Module Series
"csq" Indication of signal strength and channel bit error rate change
(similar to AT+CSQ, see Chapter 5.9). Default: OFF. If set to ON,
+QIND: "csq",<rssi>,<ber> is present.
"smsfull" SMS storage full indication. Default: OFF. If set to ON,
+QIND: "smsfull",<storage> is present.
"ring" RING indication. Default: ON.
"smsincoming" Incoming message indication. Default: ON. Related URC list:
+CMTI, +CMT, +CDS
"act" Indication of network access technology change. Default: OFF. If
set to ON, +QIND: "act",<actvalue> is present immediately. Only
when the network access technology changes, a new URC is
reported.
<actvalue> is a string type value. Its values are listed below:
"WCDMA"
"HSDPA"
"HSUPA"
"HSDPA&HSUPA"
"LTE"
"UNKNOWN" (MT not registered on network)
URC examples:
+QIND: "act","HSDPA&HSUPA"
+QIND: "act","UNKNOWN"
<enable> Integer type. URC indication is ON or OFF.
0 OFF
1 ON
<savetonvram> Integer type. Whether to save configuration into NVM.
0 Do not save
1 Save
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+QINDCFG=<URC_type>,<enable>,1 writes data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 53 / 282

5G Module Series
4
(U)SIM-Related Commands
4.1. AT+CIMI Request IMSI
This command requests the IMSI (International Mobile Subscriber Identity), which is intended to permit
TE to identify the individual (U)SIM card or active application in UICC (GSM or (U)SIM) that is attached
to MT.
AT+CIMI Request IMSI
Test Command Response
AT+CIMI=? OK
Execution Command Response
AT+CIMI TA returns <IMSI> for identifying the individual (U)SIM
attached to the module.
<IMSI>
OK
If there is any error :
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<IMSI> String type without double quotes. International mobile subscriber identity.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CIMI //Query IMSI number of (U)SIM attached to MT.
460023210226023 //IMSI number of (U)SIM attached to MT.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 54 / 282

5G Module Series
4.2. AT+ICCID Get ICCID
This command returns ICCID (Integrated Circuit Card ID) if a (U)SIM card is inserted.
AT+ICCID Query ICCID
Test Command Response
AT+ICCID=? OK
Execution Command Response
AT+ICCID +ICCID: <ICCID>
OK
If there is any error:
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Parameter
<ICCID> String type without double quotes. ICCID of (U)SIM card.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+ICCID //Query ICCID of (U)SIM card.
+ICCID: 89148000000000000002
OK
4.3. AT+CLCK Facility Lock
This command locks/unlocks or interrogates a MT or a network facility <fac>. Normally such actions
require a password. When querying the status of a network service (<mode>=2), the response line for
"not active" case (<status>=0) should be returned only if the service is not active for any <class>.
AT+CLCK Facility Lock
Test Command Response
AT+CLCK=? +CLCK: (list of supported <fac>s)
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 55 / 282

5G Module Series
OK
Write Command Response
AT+CLCK=<fac>,<mode>[,<passwor If <mode> is not 2 and the command is executed
d>[,<class>]] successfully:
OK
If <mode>=2 and the command is executed successfully:
+CLCK: <status>[,<class>]
[+CLCK: <status>[,<class>]]
[…]
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 5 s
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Reference 3GPP TS 27.007
Parameter
<fac> String type.
"SC" (U)SIM (lock (U)SIM/UICC card inserted in the currently selected card slot)
(U)SIM/UICC requests the password at MT power-up and when this lock
command is issued).
"AO" BAOC (Barr All Outgoing Calls, see 3GPP TS 22.088).
"OI" BOIC (Barr Outgoing International Calls, see 3GPP TS 22.088).
"OX" BOIC-exHC (Barr Outgoing International Calls except to Home Country, see
3GPP TS 22.088).
"AI" BAIC (Barr All Incoming Calls, see 3GPP TS 22.088).
"IR" BIC-Roam (Barr Incoming Calls when Roaming outside the home country, see
3GPP TS 22.088).
"AB" All barring services (see 3GPP TS 22.030, applicable only for <mode>=0).
"AG" All outgoing barring services (see 3GPP TS 22.030, applicable only for
<mode>=0).
"AC" All incoming barring services (see 3GPP TS 22.030, applicable only for
<mode>=0).
"FD" (U)SIM card or active application in UICC (GSM or (U)SIM) fixed dialing
memory feature (if SIM PIN2 authentication has not been performed during
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 56 / 282

                                          5G Module Series
    the current session, SIM PIN2 is required as <password>. See Chapter 4.4
|     |     | for details about SIM PIN2).  |
| --- | --- | ----------------------------- |
"PF"   Lock Phone to the very first inserted (U)SIM/UICC card (also referred in the
    present document as PH-FSIM). MT requests a password when other
|         |                                | (U)SIM/UICC cards are inserted.                         |
| ------- | ------------------------------ | ------------------------------------------------------- |
|         | "PN"                           | Network Personalization (see 3GPP TS 22.022).           |
|         | "PU"                           | Network Subset Personalization (see 3GPP TS 22.022).    |
|         | "PP"                           | Service Provider Personalization (see 3GPP TS 22.022).  |
|         | "PC"                           | Corporate Personalization (see 3GPP TS 22.022).         |
| <mode>  | Integer type. Operation mode.  |                                                         |
0  Unlock
1  Lock
2  Query status
| <password>  | String type. Password.  |     |
| ----------- | ----------------------- | --- |
| <class>     | Integer type.           |     |
1  Voice
2  Data
4  Fax
7  All telephony except SMS
8  Short message service
16  Data circuit synchronization
32  Data circuit asynchronization
| <status>  | Integer type. Lock status.  |     |
| --------- | --------------------------- | --- |
0  OFF
1  ON

NOTE
When <mode> is not 2, executing AT+CLCK=<fac>,<mode>[,<password>[,<class>]] writes data to
NVM. Please proceed with caution.

Example
| AT+CLCK="SC",2  |       |   //Query (U)SIM card status.       |
| --------------- | ----- | ----------------------------------- |
| +CLCK: 0        |       |   //(U)SIM card is unlocked (OFF).  |

OK
| AT+CLCK="SC",1,"1234"   |     | //Lock (U)SIM card. Password: 1234.  |
| ----------------------- | --- | ------------------------------------ |
OK
| AT+CLCK="SC",2  |       |   //Query (U)SIM card status.    |
| --------------- | ----- | -------------------------------- |
| +CLCK: 1        |       |   //(U)SIM card is locked (ON).  |

OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                57 / 282

5G Module Series
AT+CLCK="SC",0,"1234" //Unlock (U)SIM card.
OK
4.4. AT+CPIN Enter PIN
This command sends to the MT a password that is necessary before it can be operated, or queries
whether MT requires a password before it can be operated. The password may be (U)SIM PIN, (U)SIM
PUK (PIN Unlocking Key), PH-SIM PIN, etc.
AT+CPIN Enter PIN
Test Command Response
AT+CPIN=? OK
Read Command Response
AT+CPIN? MT returns an alphanumeric string indicating if a password is
required.
+CPIN: <code>
OK
If there is any error related to MT functionality:
+CME ERROR: <err>
Write Command Response
AT+CPIN=<pin>[,<new_pin>] MT stores a password, such as (U)SIM PIN, (U)SIM PUK,
required to operate it. If the PIN is to be entered twice, MT
automatically repeats the PIN. If no PIN request is pending,
no action will be taken and an error message +CME ERROR
is returned to TE.
If the PIN required is (U)SIM PUK or (U)SIM PUK2, the
second pin is required. The second PIN <new_pin> replaces
the old pin in the (U)SIM.
OK
If there is any error:
+CME ERROR: <err>
Maximum Response Time 5 s
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 58 / 282

                                          5G Module Series
Parameter
| <code>  | String type without double quotes.  |     |                                     |     |     |
| ------- | ----------------------------------- | --- | ----------------------------------- | --- | --- |
|         | READY                               |     | MT is not pending for any password  |     |     |
|         | SIM PIN                             |     | MT is waiting for (U)SIM PIN        |     |     |
|         | SIM PUK                             |     | MT is waiting for (U)SIM PUK        |     |     |
|         | SIM PIN2                            |     | MT is waiting for (U)SIM PIN2       |     |     |
|         | SIM PUK2                            |     | MT is waiting for (U)SIM PUK2       |     |     |
PH-NET PIN     MT is waiting for network personalization password
PH-NET PUK    MT is waiting for network personalization unlocking password
PH-NETSUB PIN   MT is waiting for network subset personalization password
PH-NETSUB PUK  MT is waiting for network subset personalization unlocking
|     |     |       | password        |       |      |
| --- | --- | ----- | --------------- | ----- | ---- |
PH-SP PIN      MT is waiting for service provider personalization password
PH-SP PUK      MT is waiting for service provider personalization unlocking
|     |     |       | password   |     |     |
| --- | --- | ----- | ---------- | --- | --- |
PH-CORP PIN    MT is waiting for corporate personalization password
PH-CORP PUK    MT is waiting for corporate personalization unlocking
|     |     |       | password  |     |     |
| --- | --- | ----- | --------- | --- | --- |
<pin>  String type. Password. If the requested password is a PUK, such as (U)SIM PUK, (U)SIM
PUK2, etc., then <pin> must be followed by <new_pin>.
<new_pin>  String type. A second PIN to replace the old PIN in (U)SIM.
| <err>  | Error code. For more details, see Chapter 13.5.  |     |     |     |     |
| ------ | ------------------------------------------------ | --- | --- | --- | --- |
Example
//Enter PIN
AT+CPIN?                //Whether or not a password is required.
+CPIN: SIM PIN              //Waiting for (U)SIM PIN to be entered.

OK
| AT+CPIN="1234"  |     |       |   //Enter PIN.  |     |     |
| --------------- | --- | ----- | --------------- | --- | --- |
OK

+CPIN: READY
| AT+CPIN?  |       |       |   //PIN has already been entered.  |     |     |
| --------- | ----- | ----- | ---------------------------------- | --- | --- |
+CPIN: READY

OK
//Enter PUK and PIN
AT+CPIN?                //Whether or not a password is required.
+CPIN: SIM PUK             //Waiting for (U)SIM PUK to be entered.

OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                59 / 282

5G Module Series
AT+CPIN="26601934","1234" //Enter PUK and the new password.
OK
+CPIN: READY
AT+CPIN? //Whether or not a password is required.
+CPIN: READY //PUK has already been entered.
OK
4.5. AT+CPWD Change Password
This command sets a new password for the facility lock function defined by AT+CLCK.
AT+CPWD Change Password
Test Command Response
AT+CPWD=? MT returns a list of pairs that present the available
facilities and the maximum length of their passwords.
+CPWD: list of supported (<fac>,<pwdlength>)s
OK
Write Command Response
AT+CPWD=<fac>,<oldpwd>,<newpwd> OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 5 s
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Reference 3GPP TS 27.007
Parameter
<fac> String type. Facility lock type.
"SC" (U)SIM (lock (U)SIM/UICC card inserted in the currently selected card slot)
(U)SIM/UICC requests the password at MT power-up and when this lock
command is issued).
"AO" BAOC (Barr All Outgoing Calls, see 3GPP TS 22.088).
"OI" BOIC (Barr Outgoing International Calls, see 3GPP TS 22.088).
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 60 / 282

                                          5G Module Series
"OX"  BOIC-exHC (Barr Outgoing International Calls except to Home Country, see
|     |        | 3GPP TS 22.088).                                     |     |
| --- | ------ | ---------------------------------------------------- | --- |
|     | "AI"   | BAIC (Barr All Incoming Calls, see 3GPP TS 22.088).  |     |
"IR"   BIC-Roam (Barr Incoming Calls when Roaming outside the home country, see
|     |     | 3GPP TS 22.088).  |     |
| --- | --- | ----------------- | --- |
"AB"  All barring services (see 3GPP TS 22.030, applicable only for <mode>=0).
"AG"  All outgoing barring services (see 3GPP TS 22.030, applicable only for
|     |     | <mode>=0).  |     |
| --- | --- | ----------- | --- |
"AC"  All incoming barring services (see 3GPP TS 22.030, applicable only for
|     |     | <mode>=0).  |     |
| --- | --- | ----------- | --- |
"FD"  (U)SIM card or active application in UICC (GSM or (U)SIM) fixed dialing
    memory feature (if SIM PIN2 authentication has not been performed during
    the current session, SIM PIN2 is required as <password>. See Chapter 4.4
|     |     | for details about SIM PIN2).  |     |
| --- | --- | ----------------------------- | --- |
"PF"   Lock Phone to the very first inserted (U)SIM/UICC card (also referred in the
    present document as PH-FSIM). MT requests a password when other
|              |                                         | (U)SIM/UICC cards are inserted.                         |     |
| ------------ | --------------------------------------- | ------------------------------------------------------- | --- |
|              | "PN"                                    | Network Personalization (see 3GPP TS 22.022).           |     |
|              | "PU"                                    | Network Subset Personalization (see 3GPP TS 22.022).    |     |
|              | "PP"                                    | Service Provider Personalization (see 3GPP TS 22.022).  |     |
|              | "PC"                                    | Corporate Personalization (see 3GPP TS 22.022).         |     |
| <pwdlength>  | Integer type. Maximum password length.  |                                                         |     |
<oldpwd>  String type. Password specified for the facility from the user interface or with command.
| <newpwd>  | String type. New password.                       |     |     |
| --------- | ------------------------------------------------ | --- | --- |
| <err>     | Error code. For more details, see Chapter 13.5.  |     |     |
Example
AT+CPIN?                //Whether or not a password is required.
+CPIN: READY

OK
AT+CPWD="SC","1234","4321"      //Change (U)SIM card password to "4321".
OK
//Restart MT or re-activate the (U)SIM card
| AT+CPIN?  |       |       |   //Waiting (U)SIM PIN to be entered.  |
| --------- | ----- | ----- | -------------------------------------- |
+CPIN: SIM PIN

OK
AT+CPIN="4321"            //PIN must be entered to define a new password "4321".
OK

+CPIN: READY
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                61 / 282

5G Module Series
4.6. AT+CSIM Generic (U)SIM Access
This command allows a direct control of the (U)SIM inserted in the selected card slot by a remote
application on TE. TE should then keep the processing of (U)SIM information within the frame specified
by GSM/UMTS.
AT+CSIM Generic (U)SIM Access
Test Command Response
AT+CSIM=? OK
Write Command Response
AT+CSIM=<length>,<command> +CSIM: <length>,<response>
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are not saved.
Reference 3GPP TS 27.007
Parameter
<length> Integer type. Length of <command> or <response>. Unit: byte.
<command> String type in hexadecimal format. Command transferred by the MT to the (U)SIM in
the format described in 3GPP TS 51.011.
<response> String type in hexadecimal format. Response to the command transferred by the
(U)SIM to the MT in the format described in 3GPP TS 51.011.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CSIM=?
OK
AT+CSIM=10,"80F2010112"
+CSIM: 40,"8410A0000000871002FF86FF0389FFFFFFFF9000"
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 62 / 282

5G Module Series
4.7. AT+CRSM Restricted (U)SIM Access
This command offers easy and limited access to the (U)SIM database. It transmits the (U)SIM command
number <command> and its required parameters to MT.
AT+CRSM Restricted (U)SIM Access
Test Command Response
AT+CRSM=? OK
Write Command Response
AT+CRSM=<command>[,<fileld>[,<P +CRSM: <sw1>,<sw2>[,<response>]
1>,<P2>,<P3>[,<data>][,<pathld>]]]
OK
If there is any error:
+CME ERROR: <err>
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are not saved.
Reference 3GPP TS 27.007
Parameter
<command> Integer type. (U)SIM command number.
176 READ BINARY
178 READ RECORD
192 GET RESPONSE
214 UPDATE BINARY
220 UPDATE RECORD
242 STATUS
203 RETRIEVE DATA
219 SET DATA
<fileId> Integer type. Identifier for an elementary data file on (U)SIM, if used by
<command>.
<P1>, <P2>, Parameters passed on by the MT to the (U)SIM. These parameters are mandatory
<P3> for every command, except GET RESPONSE and STATUS. Their values are
described in 3GPP TS 51.011.
<data> String type in hexadecimal format. Information to be written to the (U)SIM.
<pathId> String type in hexadecimal format. Directory path of an elementary file on a
(U)SIM/UICC.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 63 / 282

5G Module Series
<sw1>, <sw2> Integer type. Information from the (U)SIM about the execution of the actual
command. These parameters are delivered to the TE in both cases, on
successful or failed execution of the command.
<response> String type in hexadecimal format. Response of a successful completion of the
previously issued command. STATUS and GET RESPONSE return data, which
gives information about the current elementary data field. The information includes
the type and size of the file (see 3GPP TS 51.011). After READ BINARY, READ
RECORD or RETRIEVE DATA command, the requested data will be returned.
<response> is not returned after a successful UPDATE BINARY, UPDATE
RECORD or SET DATA command.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CRSM=?
OK
AT+CRSM=242
+CRSM: 144,0,"623A8202782183027FF08410A0000000871002FF86FFFF89FFFFFFFF8A01058B032
F0601C61290017883010183018183010A83010B83010C81026DA7"
OK
AT+CRSM=242,80,01,01,12
+CRSM: 144,0,"8410A0000000871002FF86FF"
OK
4.8. AT+CCHO Open Logical Channel
This command opens a logical channel. <sessionid> is to be used when you send commands with
generic UICC logical channel access AT+CGLA (see Chapter 4.10).
AT+CCHO Open Logical Channel
Test Command Response
AT+CCHO=? OK
Write Command Response
AT+CCHO=<dfname> +CCHO: <sessionid>
OK
If there is any error:
+CME ERROR: <err>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 64 / 282

5G Module Series
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is not saved.
Reference 3GPP TS 27.007
Parameter
<sessionid> Integer type. A session ID to be used to target a specific application on the smart card,
e.g. (U)SIM, WIM, iSIM, using the logical channel mechanism.
<dfname> String Type. All selectable applications in the UICC referenced by a DF name coded on
1 to 16 bytes.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CCHO=?
OK
AT+CCHO="A0000000871002FF86FFFF89FFFFFFFF" //Open a logical channel.
+CCHO: 1
OK
4.9. AT+CCHC Close Logical Channel
This command asks ME to close a communication session with the active UICC, then ME will close the
previously opened logical channel, and TE will no longer be able to send commands on this logical
channel. UICC closes the logical channel after receiving this command.
AT+CCHC Close Logical Channel
Test Command Response
AT+CCHC=? OK
Write Command Response
AT+CCHC=<sessionid> OK
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is not saved.
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 65 / 282

5G Module Series
Parameter
<sessionid> Integer type. A session ID to be used to target a specific application on the smart,
card, e.g. (U)SIM, WIM, iSIM, using logical channel mechanism.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CCHC=?
OK
AT+CCHC=1 //Close logical channels.
OK
4.10. AT+CGLA Generic UICC Logical Channel Access
This command allows a direct control of the currently selected UICC by a remote application on TE. TE
will process UICC information within the frame specified by GSM/UMTS.
AT+CGLA Generic UICC Logical Channel Access
Test Command Response
AT+CGLA=? OK
Write Command Response
AT+CGLA=<sessionid>,<length>,<command> +CGLA: <length>,<response>
OK
If there is any error:
+CME ERROR: <err>
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are not saved.
Reference 3GPP TS 27.007
Parameter
<sessionid> Integer type. Identifier of the session used to send the APDU commands to the
UICC. It is mandatory to send commands to the UICC when targeting applications
on the smart card using a logical channel other than the default channel (channel
"0").
<length> Integer type. Length of the characters that are sent to TE in <command> or
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 66 / 282

5G Module Series
<response> (two times the actual length of the command or response).
<command> String type in hexadecimal character format. Command passed on by the MT to the
UICC as described in 3GPP TS 31.101.
<response> String type in hexadecimal character format. Response to the command passed on
by the UICC to the MT in the format as described in 3GPP TS 31.101.
<err> Error code. For more details, see Chapter 13.5.
NOTE
Before using this command, the logical channel must be opened via AT+CCHO=<dfname>.
Example
AT+CGLA=?
OK
AT+CGLA=1,10,"80F2010112"
+CGLA: 40,"8410A0000000871002FF86FFFF89FFFFFFFF9000"
OK
4.11. AT+QPINC Display PIN Remainder Counter
This command queries the number of attempts left to enter the password of (U)SIM PIN/PUK.
AT+QPINC Display PIN Remainder Counter
Test Command Response
AT+QPINC=? +QPINC: (list of supported <facility>s)
OK
Read Command Response
AT+QPINC? +QPINC: "SC",<pincounter>,<pukcounter>
+QPINC: "P2",<pincounter>,<pukcounter>
OK
Write Command Response
AT+QPINC=<facility> +QPINC: <facility>,<pincounter>,<pukcounter>
OK
If there is any error:
+CME ERROR: <err>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 67 / 282

5G Module Series
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<facility> String type.
"SC" (U)SIM PIN
"P2" (U)SIM PIN2
<pincounter> Integer type. Number of attempts left to enter PIN.
<pukcounter> Integer type. Number of attempts left to enter PUK.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+QPINC?
+QPINC: "SC",3,10
+QPINC: "P2",3,10
OK
4.12. AT+QINISTAT Query Initialization Status of (U)SIM Card
This command queries the initialization status of (U)SIM card.
AT+QINISTAT Query Initialization Status of (U)SIM Card
Test Command Response
AT+QINISTAT=? +QINISTAT: (list of supported <status>s)
OK
Execution Command Response
AT+QINISTAT +QINISTAT: <status>
OK
Maximum Response Time 300 ms
Characteristics -
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 68 / 282

5G Module Series
Parameter
<status> Integer type. Initialization status of (U)SIM card. Actual value is the sum of several of the
following four states (e.g. 7 = 1 + 2 + 4 means CPIN READY + SMS DONE + PB DONE).
0 Initial state
1 CPIN READY. Operation like locking/unlocking PIN is allowed.
2 SMS DONE. SMS initialization completed.
4 PB DONE. Phonebook initialization completed.
Example
AT+QINISTAT
+QINISTAT: 7
OK
4.13. AT+QSIMDET (U)SIM Card Detection
This command enables or disables (U)SIM card detection. (U)SIM card is detected by GPIO interrupt.
The level of (U)SIM card detection pin should also be set when the (U)SIM card is inserted.
AT+QSIMDET (U)SIM Card Detection
Test Command Response
AT+QSIMDET=? +QSIMDET: (list of supported <enable>s),(list of supported
<insert_level>s)
OK
Read Command Response
AT+QSIMDET? +QSIMDET: <enable>,<insert_level>
OK
Write Command Response
AT+QSIMDET=<enable>,<insert_level> OK
Or
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configurations are saved automatically.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 69 / 282

5G Module Series
Parameter
<enable> Integer type. Enable or disable (U)SIM card detection.
0 Disable
1 Enable
<insert_level> Integer type. Level of (U)SIM card detection pin when a (U)SIM card is inserted.
0 Low level
1 High level
NOTE
1. (U)SIM card detection is invalid if the configured value of <insert_level> is inconsistent with
hardware design.
2. The configuration of <insert_level> is valid only when (U)SIM card detection is enabled.
3. Executing AT+QSIMDET=<enable>,<insert_level> writes data to NVM. Please proceed with
caution.
Example
AT+QSIMDET=1,0 //Set (U)SIM card detection pin level to low when a (U)SIM card is inserted.
OK
<Remove (U)SIM card>
+CPIN: NOT READY
<Insert (U)SIM card>
+CPIN: READY
4.14. AT+QSIMSTAT (U)SIM Card Insertion Status Report
This command queries (U)SIM card insertion status or determines whether (U)SIM card insertion status
report is enabled.
AT+QSIMSTAT (U)SIM Card Insertion Status Report
Test Command Response
AT+QSIMSTAT=? +QSIMSTAT: (list of supported <enable>s)
OK
Read Command Response
AT+QSIMSTAT? +QSIMSTAT: <enable>,<inserted_status>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 70 / 282

5G Module Series
OK
Write Command Response
AT+QSIMSTAT=<enable> OK
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<enable> Integer type. Enable or disable (U)SIM insertion status report. If it is enabled, the
URC +QSIMSTAT: <enable>,<inserted_status> is reported when (U)SIM card is
inserted or removed.
0 Disable
1 Enable
<inserted_status> Integer type. (U)SIM card insertion status.
0 Removed
1 Inserted
2 Unknown (before (U)SIM initialization)
Example
AT+QSIMSTAT? //Query (U)SIM card insertion status.
+QSIMSTAT: 0,1
OK
AT+QSIMDET=1,0
OK
AT+QSIMSTAT=1 //Enable reporting of (U)SIM card insertion status.
OK
AT+QSIMSTAT? //Query (U)SIM card insertion status.
+QSIMSTAT: 1,1
OK
//Remove the (U)SIM card
+QSIMSTAT : 1,0 //Report of (U)SIM card insertion status: removed.
+CPIN: NOT READY
AT+QSIMSTAT? //Query (U)SIM card insertion status.
+QSIMSTAT: 1,0
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 71 / 282

5G Module Series
//Insert a (U)SIM card
+QSIMSTAT : 1,1 //Report of (U)SIM card insertion status: inserted.
+CPIN: READY
NOTE
Executing AT+QSIMSTAT=<enable> writes data to NVM. Please proceed with caution.
4.15. AT+QUIMSLOT Switch (U)SIM Slot
This command queries the slot currently used by the (U)SIM and configures the (U)SIM slot to be used.
AT+QUIMSLOT Switch (U)SIM Slot
Test Command Response
AT+QUIMSLOT=? +QUIMSLOT: (list of supported <slot>s)
OK
Read Command Response
AT+QUIMSLOT? +QUIMSLOT: <slot>
OK
Write Command Response
AT+QUIMSLOT=<slot> OK
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<slot> Integer type. Physical (U)SIM slot.
1 (U)SIM slot 1
2 (U)SIM slot 2
NOTE
Executing AT+QUIMSLOT=<slot> writes data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 72 / 282

5G Module Series
Example
AT+QUIMSLOT? //Query currently used (U)SIM slot.
+QUSIMSLOT: 1
OK
AT+QUIMSLOT=2 //Switch to (U)SIM slot 2.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 73 / 282

5G Module Series
5
Network Service Commands
5.1. AT+COPS Operator Selection
This command returns information about the current operators and their status, and allows automatic or
manual network selection.
The Test Command returns a set or sets of five parameters, each set representing an operator present in
the network. Any of the formats may be unavailable and should then be an empty field. The list of
operators shall be in the order of: home network, networks referenced in (U)SIM and other networks.
The Read Command returns the current network registration/deregistration mode and the currently
selected operator. If no operator is selected, <format>, <oper> and <AcT> are omitted.
The Write Command forces an attempt to select and register the GSM/UMTS/EPS/5G network operator.
If the selected operator is not available, no other operator shall be selected (except <mode>=4). The
format of selected operator name shall apply to further Read Command (AT+COPS?).
AT+COPS Operator Selection
Test Command Response
AT+COPS=? +COPS: [list of supported (<stat>,long alphanumeric <oper>,short
alphanumeric <oper>,numeric <oper> [,<AcT>])s][,,(list of
supported <mode>s),(list of supported <format>s)]
OK
If there is any error:
+CME ERROR: <err>
Read Command Response
AT+COPS? +COPS: <mode>[,<format>[,<oper>][,<AcT>]]
OK
If there is any error related to MT functionality:
+CME ERROR: <err>
Write Command Response
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 74 / 282

5G Module Series
AT+COPS=<mode>[,<format> OK
[,<oper>[,<AcT>]]] Or
+CME ERROR: <err>
Maximum Response Time 180 s, determined by the network.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<stat> Integer type. Availability of operators.
0 Unknown
1 Operator available
2 Current operator
3 Operator forbidden
<oper> String type. Operator in format as per <format>.
<mode> Integer type.
0 Automatic operator selection (<oper> field is ignored).
1 Manual operator selection (<oper> field shall be present and <AcT> is optional)
2 Deregistration from network
3 Set only <format> (for AT+COPS? Read Command), and do not attempt
registration/deregistration (<oper> and <AcT> fields are ignored). This value is
invalid in the response of the Read Command.
4 Manual/automatic selection (<oper> field shall be present). If manual selection
fails, automatic mode (<mode>=0) is entered.
<format> Integer type. Format of <oper>.
0 Long format alphanumeric <oper> up to 16 characters.
1 Short format alphanumeric <oper>.
2 Numeric <oper>. GSM location area identification number.
<AcT> Integer type. Access technology selected. Values 4, 5, 6 occur only in the response of the
Read Command while MS is in data service state, and they are not intended for the Write
Command of AT+COPS.
2 UTRAN
4 UTRAN W/HSDPA
5 UTRAN W/HSUPA
6 UTRAN W/HSDPA and HSUPA
7 E-UTRAN
10 E-UTRAN connected to 5GCN
11 NR connected to 5GCN
12 NG-RAN
13 E-UTRAN-NR dual connectivity
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 75 / 282

5G Module Series
NOTE
1. When selecting 5G SA network, <AcT> should be set to 12, and when registering 5G SA network,
<AcT> returned by AT+COPS? is 11.
2. Executing AT+COPS=<mode>[,<format>[,<oper>[,<AcT>]]] writes data to NVM. Please proceed
with caution.
Example
AT+COPS=? //List all network operators present in the network.
+COPS: (1,"CHN-UNICOM","UNICOM","46001",2),(1,"CHN-UNICOM","UNICOM","46001",12),(3,"C
HINA MOBILE","CMCC","46000",7),(3,"CHN-CT","CT","46011",12),(3,"CHN-CT","CT","46011",7),(3,
"CHINA MOBILE","CMCC","46000",12),,(0-4),(0-2)
OK
AT+COPS? //Query the currently selected network operator.
+COPS: 0,0,"CHINA MOBILE",13
OK
5.2. AT+CREG Network Registration Status
The Read Command returns the presentation of URC (Unsolicited Result Code) and an integer <stat>
which shows whether the network has currently indicated the registration of MT. Location information
parameters <lac> and <ci> are returned only when <n>=2 and MT is registered on the network.
The Write Command sets whether to return an URC or not and controls the presentation of URC +CREG:
<stat> when <n>=1 and there is a change in the MT network registration status.
AT+CREG Network Registration Status
Test Command Response
AT+CREG=? +CREG: (list of supported <n>s)
OK
Read Command Response
AT+CREG? +CREG: <n>,<stat>[,<lac>,<ci>[,<AcT>]]
OK
If there is any error:
+CME ERROR: <err>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 76 / 282

                                          5G Module Series
Write Command  Response
AT+CREG=[<n>]  OK
Maximum Response Time  300 ms
Characteristics  -
Reference  3GPP TS 27.007
Parameter
| <n>         |   Integer type   |                                                 |
| ----------- | ---------------- | ----------------------------------------------- |
|             |   0              | Disable network registration URC                |
|             |    1             | Enable network registration URC: +CREG: <stat>  |
  2    Enable network registration and location information URC:
 +CREG: <stat>[,<lac>,<ci>[,<AcT>]]
| <stat>    |   Integer type. Circuit mode registration status.  |     |
| --------- | -------------------------------------------------- | --- |
        0    Not registered. MT is not currently searching a new operator to register to.
|       |   1    | Registered. Home network.  |
| ----- | ------ | -------------------------- |
        2    Not registered. MT is currently searching a new operator to register to.
|       |   3     | Registration denied.  |
| ----- | ------- | --------------------- |
|       |   4     | Unknown               |
|       |    5    | Registered. Roaming.  |
<lac>        String type in hexadecimal format. Two-byte location area code.
<ci>           String type in hexadecimal format. 28-bit (UMTS/LTE) cell ID.
| <AcT>         |   Integer type. Access technology selected.  |                          |
| ------------- | -------------------------------------------- | ------------------------ |
|               |   2                                          | UTRAN                    |
|               |   4                                          | UTRAN w/HSDPA            |
|               |   5                                          | UTRAN w/HSUPA            |
|               |    6                                         | UTRAN w/HSDPA and HSUPA  |
|               |   7                                          | E-UTRAN                  |
              10    E-UTRAN connected to 5GCN (not supported currently)
                  11    NR connected to 5GCN (not supported currently)
|                 |   12                                               | NG-RAN (not supported currently)  |
| --------------- | -------------------------------------------------- | --------------------------------- |
|                 |   13                                               | E-UTRAN-NR dual connectivity      |
| <err>           |   Error code. For more details, see Chapter 13.5.  |                                   |
Example
AT+CREG=1
OK

+CREG: 1           //URC reports that MT has registered on network.
| AT+CREG=2   |     |    //Activate extended URC mode.  |
| ----------- | --- | --------------------------------- |
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                77 / 282

5G Module Series
+CREG: 1,"D509","80D413D",7 //URC reports that operator has found location area code and cell ID.
5.3. AT+CGREG PS Network Registration Status
This command queries the PS network registration status and controls the presentation of URC
+CGREG: <stat> when <n>=1 and there is a change in the MT’s GPRS network registration status in
GERAN/UTRAN, or URC +CGREG: <stat>[,[<lac>],[<ci>],[<AcT>],[<rac>]] when <n>=2 and there is a
change of the network cell in GERAN/UTRAN.
AT+CGREG PS Network Registration Status
Test Command Response
AT+CGREG=? +CGREG: (list of supported <n>s)
OK
Read Command Response
AT+CGREG? +CGREG: <n>,<stat>[,[<lac>],[<ci>],[<AcT>],[<rac>]]
OK
Write Command Response
AT+CGREG=[<n>] OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<n> Integer type.
0 Disable network registration URC.
1 Enable network registration URC: +CGREG:<stat>
2 Enable network registration and location information URC
+CGREG: <stat>[,[<lac>],[<ci>],[<AcT>],[<rac>]]
<stat> Integer type. GPRS registration status.
0 Not registered. MT is not currently searching an operator to register to. The UE is
in GMM state GMM-NULL or GMM-DEREGISTERED-INITIATED. The GPRS
service is disabled; the UE is allowed to attach for GPRS if requested by the user.
1 Registered. Home network. The UE is in GMM state GMM-REGISTERED or
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 78 / 282

5G Module Series
GMM-ROUTING-AREA-UPDATING-INITIATED INITIATED on the home PLMN.
2 Not registered. MT is currently trying to attach or searching an operator to register to.
The UE is in GMM state GMM-DEREGISTERED or GMM-REGISTERED-INITIATED.
The GPRS service is enabled, but an allowable PLMN is currently not available. The
UE will start a GPRS attach as soon as an allowable PLMN is available.
3 Registration denied. The UE is in GMM state GMM-NULL. The GPRS service is
disabled; and the UE is not allowed to attach for GPRS if requested by the user.
4 Unknown
5 Registered. Roaming.
<lac> String type. Two-byte location area code in hexadecimal format (e.g., "00C3" equals 195 in
decimal).
<ci> String type. Four-byte (UMTS/LTE) cell ID in hexadecimal format.
<AcT> Access technology selected.
2 UTRAN
4 UTRAN W/HSDPA
5 UTRAN W/HSUPA
6 UTRAN W/HSDPA and HSUPA
<rac> String type. One-byte routing area code in hexadecimal format.
Example
AT+CGREG=?
+CGREG: (0-2)
OK
AT+CGREG=2
OK
AT+CGREG?
+CGREG: 2,1,"D5D5","8054BBF",2,"0"
OK
+CGREG: 1,"D5D5","8054BBF",2,"0"
5.4. AT+CEREG EPS Network Registration Status
This command queries the network registration status and controls the presentation of URC +CEREG:
<stat> when <n>=1 and there is a change in the MT’s EPS network registration status in E-UTRAN, or
URC +CEREG: <stat>[,[<tac>],[<ci>],[<AcT>]] when <n>=2 and there is a change of the network cell
in E-UTRAN.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 79 / 282

5G Module Series
AT+CEREG EPS Network Registration Status
Test Command Response
AT+CEREG=? +CEREG: (list of supported <n>s)
OK
Read Command Response
AT+CEREG? +CEREG: <n>,<stat>[,<tac>,<ci>[,<AcT>]]
OK
Write Command Response
AT+CEREG=[<n>] OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<n> Integer type.
0 Disable network registration URC
1 Enable network registration URC +CEREG:<stat>
2 Enable network registration and location information URC
+CEREG: <stat>[,[<tac>],[<ci>],[<AcT>]]
<stat> Integer type. EPS registration status.
0 Not registered, MT is not currently searching an operator to register to.
1 Registered. Home network.
2 Not registered, but MT is currently trying to attach or searching an operator to
register to.
3 Registration denied.
4 Unknown
5 Registered. Roaming.
<tac> String type. Two-byte tracking area code in hexadecimal format.
<ci> String type. Four-byte (E-UTRAN) cell ID in hexadecimal format.
<AcT> Integer type. Access technology selected.
7 E-UTRAN
13 E-UTRAN-NR dual connectivity
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 80 / 282

5G Module Series
Example
AT+CEREG=?
+CEREG: (0-2)
OK
AT+CEREG=2
OK
AT+CEREG?
+CEREG: 2,1,"DE10","5A29C0B",7
OK
+CEREG: 1,"DE10","5A29C0B",7
5.5. AT+C5GREG 5GS Network Registration Status
This command queries the network registration status and controls the presentation of following URC.
⚫ +C5GREG: <stat> is reported when <n>=1 and there is a change in the module's network
registration status in 5GS,
⚫ +C5GREG: <stat>[,[<tac>],[<ci>],[<AcT>],[<Allowed_NSSAI_length>],[<Allowed_NSSAI>]] is
reported when <n>=2 and there is a change of the network cell in 5GS or the network provides an
Allowed NSSAI. The parameters <AcT>, <tac>, <ci>, <Allowed_NSSAI_length> and
<Allowed_NSSAI> are provided only if available.
AT+C5GREG 5GS Network Registration Status
Test Command Response
AT+C5GREG=? +C5GREG: (list of supported <n>s)
OK
Read Command Response
AT+C5GREG? +C5GREG: <n>,<stat>[,[<tac>],[<ci>],[<AcT>],[<Allowed_
NSSAI_length>],[<Allowed_NSSAI>]]
OK
Write Command Response
AT+C5GREG=[<n>] OK
Or
ERROR
Maximum Response Time 300 ms
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 81 / 282

                                          5G Module Series
Characteristics  -
Reference  3GPP TS 27.007
Parameter
| <n>     | Integer type.                                       |     |
| ------- | --------------------------------------------------- | --- |
|         | 0  Disable network registration URC                 |     |
|         | 1   Enable network registration URC +C5GREG:<stat>  |     |
      2  Enable network registration and location information URC +C5GREG:
      <stat>[,[<tac>],[<ci>],[<AcT>],[<Allowed_NSSAI_length>],[<Allowed_NSSAI>]]
| <stat>      | Integer type. NR registration status.  |     |
| ----------- | -------------------------------------- | --- |
      0   Not registered. MT is currently not searching an operator to registerto.
|          | 1   Registered. Home network.  |     |
| -------- | ------------------------------ | --- |
              2  Not registered. MT is currently trying to attach or searching an operator to register to.
|               | 3  Registration denied.                     |     |
| ------------- | ------------------------------------------- | --- |
|               | 4  Unknown                                  |     |
|               | 5  Registered. Roaming.                     |     |
|               | 8  Registered for emergency services only.  |     |
<tac>       String type. Three-byte tracking area code in hexadecimal format.
<ci>         String type. Five-byte (NR) cell ID in hexadecimal format.
| <AcT>         | Integer type. Access technology selected.  |     |
| ------------- | ------------------------------------------ | --- |
|               | 10    E-UTRAN connected to 5GCN            |     |
|               | 11    NR connected to 5GCN                 |     |
<Allowed_NSSAI_length>  Integer type. Number of octets of the <Allowed_NSSAI> information
|       |       |   element.  |
| ----- | ----- | ----------- |
<Allowed_NSSAI>      String type in hexadecimal format. Depending on the form, the string
              can  be separated by dot(s), semicolon(s) and colon(s). This parameter
              indicates the list of allowed S-NSSAIs received from the network.
              <Allowed_NSSAI> is coded as a list of <S-NSSAI>s separated by
              colons. See <S-NSSAI> in 3GPP 27.007 subclause 10.1.1. This
              parameter is not subject to conventional character conversion as per
|       |       |   AT+CSCS.  |
| ----- | ----- | ----------- |
<S-NSSAI>        String type in hexadecimal character format. Depending on the form, the
      string can be separated by dot(s) and semicolon(s). This parameter is
      associated with the PDU session for identifying a network slice in 5GS,
      see 3GPP TS 23.501 and 3GPP TS 24.501. For the format and the
      encoding of S-NSSAI, see also 3GPP TS 23.003. This parameter is
      not subject to conventional character conversion as per AT+CSCS.
|     |     |   The parameter takes one of the following forms:  |
| --- | --- | -------------------------------------------------- |
              sst        only slice/service type (SST) is present.
              sst;mapped_sst  SST and mapped configured SST are present.
              sst.sd      SST and slice differentiator (SD) are present.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                82 / 282

                                          5G Module Series
              sst.sd;mapped_sst  SST, SD and mapped configured SST are present
              sst.sd;mapped_sst.mapped_sd   SST, SD, mapped configured SST
|       |       |       |       |     |   and mapped configured SD are   |     |     |
| ----- | ----- | ----- | ----- | --- | -------------------------------- | --- | --- |
|       |       |       |       |     |   present.                       |     |     |
Example
AT+C5GREG=?
+C5GREG: (0-2)

OK
AT+C5GREG=2
OK
AT+C5GREG?
+C5GREG: 2,1,"690E0F","9013B004",11,4,"01.000000"

OK
+C5GREG: 1,"690E0F","9013B004",11,4,"01.000000"

5.6.  AT+CGDCONT  Define PDP Context

This command specifies PDP context parameters for a specific context <cid>. A special form of the Write
Command (AT+CGDCONT=<cid>) causes the values for <cid> to become undefined. It is not allowed
to change the definition of an already activated context.

This Read Command returns the current configurations for each defined PDP context.
AT+CGDCONT  Define PDP Context
| Test Command  |     |     |     | Response                                   |                   |                |            |
| ------------- | --- | --- | --- | ------------------------------------------ | ----------------- | -------------- | ---------- |
| AT+CGDCONT=?  |     |     |     | +CGDCONT:                                  | (list             | of             | supported  |
|               |     |     |     | <cid>s),<PDP_type>,<APN>,<PDP_addr>,(list  |                   |                | of         |
|               |     |     |     | supported                                  | <d_comp>s),(list  | of             | supported  |
|               |     |     |     | <h_comp>s)[,(list                          |                   | of             | supported  |
|               |     |     |     | <IPv4AddrAlloc>s)[,(list                   |                   | of             | supported  |
|               |     |     |     | <request_type>s)[,(list                    |                   | of             | supported  |
|               |     |     |     | <SSC_mode>s)[,(list                        |                   | of  supported  | <S-        |
|               |     |     |     | NSSAI>s)[,(list                            |                   | of             | supported  |
<Pref_access_type>s)[,(list of supported <Always-
on_req>s)]]]]]]

OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                83 / 282

5G Module Series
Read Command Response
AT+CGDCONT? +CGDCONT: <cid>,<PDP_type>,<APN>,<PDP_add
r>,<d_comp>,<h_comp>[,<IPv4AddrAlloc>[,<requ
est_type>,,,,,,,,[,<SSC_mode>[,<S-NSSAI>[,<Pref_
access_type>,,[,<Always-on_req>]]]]]]
[…]
OK
Write Command
AT+CGDCONT=[<cid>[,<PDP_type>[,<APN>
Response
[,<PDP_addr>[,<d_comp>[,<h_comp>[,<IPv4
OK
AddrAlloc>[,<request_type>,,,,,,,,[,<SSC_mo
Or
de>[,<S-
ERROR
NSSAI>[,<Pref_access_type>,,[,<Always-
on_req>]]]]]]]]]]]]
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Reference 3GPP TS 27.007
Parameter
<cid> Integer type. PDP context identifier, which specifies a particular PDP context
definition. Range: 1–42. The parameter is local to the TE-MT interface and is used in
other PDP context-related commands.
<PDP_type> String type. Packet data protocol type.
"IP" IPv4. Internet protocol (see IETF STD 5)
"PPP" Point to Point protocol (see IETF STD 51)
"IPV6" Internet protocol, version 6 (see RFC 2460)
"IPV4V6" Virtual <PDP_type> introduced to handle dual IP stack UE capability
(see 3GPP TS 24.301)
<APN> String type. Access point name, which is a logical name used to select GGSN or the
external packet data network. If the value is null or omitted, the subscription value will
be requested.
<PDP_addr> String type. It identifies the MT in the address space applicable to the PDP. If the
value is null or omitted, a value may be provided by the TE during the PDP startup
procedure or, failing that, a dynamic address will be requested. The allocated address
may be read using AT+CGPADDR (see Chapter 9.4).
<d_comp> Integer type. It controls PDP data compression (applicable for SNDCP only) (see
3GPP TS 44.065).
0 Off
2 V.42bis
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 84 / 282

                                          5G Module Series
<h_comp>    Integer type. It controls PDP header compression (see 3GPP TS 44.065 and
|                  |   3GPP TS 25.323).  |          |     |     |
| ---------------- | ------------------- | -------- | --- | --- |
|                  |     0               | Off      |     |     |
|                  |     4               | RFC3095  |     |     |
<IPv4AddrAlloc>  Integer  type.  It  controls  how  the  MT/TA  requests  to  get  the  IPv4  address
|                  |        | information.                                   |     |     |
| ---------------- | ------ | ---------------------------------------------- | --- | --- |
|                  |   0    | IPv4 address allocation through NAS signaling  |     |     |
|                  |     1  | IPv4 address allocated through DHCP            |     |     |
<request_type>    Integer type. Type of PDP context activation request.
                     0  PDP context is for a new PDP context establishment or for a handover from
          a non-3GPP access network (how the MT decides whether the PDP context is for
          a new PDP context establishment or for a handover is implementation specific).
|       |     1  | PDP context is for emergency bearer services.  |     |     |
| ----- | ------ | ---------------------------------------------- | --- | --- |
<SSC_mode>    Integer type. It indicates the session and service continuity (SSC) mode for the
|       |     PDU session in 5GS, see 3GPP TS 23.501.  |                                            |     |     |
| ----- | -------------------------------------------- | ------------------------------------------ | --- | --- |
|       |     0                                        | PDU session is associated with SSC mode 1  |     |     |
|       |     1                                        | PDU session is associated with SSC mode 2  |     |     |
|       |     2                                        | PDU session is associated with SSC mode 3  |     |     |
<S-NSSAI>    String type in hexadecimal character format. Depending on the form, the string
  can be separated by dot(s) and semicolon(s). This parameter is associated with
  the PDU session for identifying a network slice in 5GS, see 3GPP TS 23.501 and
  3GPP  TS  24.501.  For  the  format  and  the  encoding  of  S-NSSAI,  see  also
  3GPP  TS  23.003.  This  parameter  is  not  subject  to  conventional  character
  conversion as per AT+CSCS. The parameter takes one of the following forms:
          sst          only slice/service type (SST) is present
          sst;mapped_sst    SST and mapped configured SST are present
          sst.sd        SST and slice differentiator (SD) are present
          sst.sd;mapped_sst  SST, SD and mapped configured SST are present
          sst.sd;mapped_sst.mapped_sd   SST, SD, mapped configured SST and
|       |       |       |       |   mapped  configured SD are present  |
| ----- | ----- | ----- | ----- | ------------------------------------ |
<Pref_access_type> Integer type. Preferred access type for the PDU session in 5GS.
|       |     See 3GPP TS 23.501and 3GPP TS 24.501.  |                                           |     |     |
| ----- | ------------------------------------------ | ----------------------------------------- | --- | --- |
|       |     0                                      | Preferred access type is 3GPP access      |     |     |
|       |     1                                      | Preferred access type is non-3GPP access  |     |     |
<Always-on_req>  Integer type. It indicates whether the UE requested to establish the PDU
           session as an always-on PDU session, see 3GPP TS 24.501.
|       |     0  | Always-on PDU session was not requested  |     |     |
| ----- | ------ | ---------------------------------------- | --- | --- |
|       |     1  | Always-on PDU session was requested      |     |     |

NOTE
Executing
AT+CGDCONT=[<cid>[,<PDP_type>[,<APN>[,<PDP_addr>[,<d_comp>[,<h_comp>[,<IPv4AddrAllo
c>[,<request_type>,,,,,,,,[,<SSC_mode>[,<S-NSSAI>[,<Pref_access_type>,,,[,<Always-
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                85 / 282

                                          5G Module Series
on_req>]]]]]]]]]]]] writes data to NVM. Please proceed with caution.

5.7.  AT+C5GNSSAI  5GS NSSAI Setting

This command enables updating the default NSSAI configuration stored at MT.
AT+C5GNSSAI  5GS NSSAI Setting
| Test Command    |     |     | Response                                   |        |     |            |
| --------------- | --- | --- | ------------------------------------------ | ------ | --- | ---------- |
| AT+C5GNSSAI=?   |     |     | +C5GNSSAI:                                 | (list  | of  | supported  |
|                 |     |     | <default_configured_nssai_length>s),(list  |        | of  | supported  |
<default_configured_nssai>s)

OK
| Read Command  |     |     | Response    |     |     |     |
| ------------- | --- | --- | ----------- | --- | --- | --- |
| AT+C5GNSSAI?  |     |     | +C5GNSSAI:  |     |     |     |
[<default_configured_nssai_length>,<default_configured
_nssai>]

OK
| Write Command                      |     |     | Response                |     |     |     |
| ---------------------------------- | --- | --- | ----------------------- | --- | --- | --- |
| AT+C5GNSSAI=<default_configured_   |     |     | OK                      |     |     |     |
| nssai_length>,<default_configured_ |     |     | If there is any error:  |     |     |     |
| nssai>                             |     |     | ERROR                   |     |     |     |
Or
+CME ERROR: <err>
| Maximum Response Time  |     |     | 300 ms          |     |     |     |
| ---------------------- | --- | --- | --------------- | --- | --- | --- |
| Characteristics        |     |     | -               |     |     |     |
| Reference              |     |     | 3GPP TS 27.007  |     |     |     |
Parameter
<default_configured_nssai_length>  Integer type. Default configured NSSAI length in octets to be
|       |       |       | stored in MT.  |     |     |     |
| ----- | ----- | ----- | -------------- | --- | --- | --- |
<default_configured_nssai>      String type in hexadecimal format. Depending on the form,
        the string can be separated by dot(s), semicolon(s) and
        colon(s). This parameter indicates the list of S-NSSAIs
        included in the default configured NSSAI to be stored in MT.
        <default_configured_nssai> is coded as a list of <S-NSSAI>s
        separated by colon(s). See <S-NSSAI> in subclause 10.1.1.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                86 / 282

5G Module Series
This parameter is not subject to conventional character
conversion as per AT+CSCS.
<err> Error code. For more details, see Chapter 13.5.
NOTE
1. If the value is an empty string (""), no default configured NSSAI is stored in MT.
2. Executing AT+C5GNSSAI=<default_configured_nssai_length>,<default_configured_nssai>
writes data to NVM. Please proceed with caution.
5.8. AT+C5GNSSAIRDP Read 5GS NSSAI Dynamic Parameters
This command returns the default configured NSSAI, rejected NSSAI for 3GPP access or non-3GPP
access stored in MT. The write command returns the default configured NSSAI, rejected NSSAI for
3GPP access and rejected NSSAI for non-3GPP access stored at the MT, if any, as well as the
configured NSSAI, allowed NSSAI for 3GPP access and allowed NSSAI for non-3GPP access stored at
the MT, if any for the PLMN identified by <plmn_id>.
AT+C5GNSSAIRDP Read 5GS NSSAI Dynamic Parameters
Test Command Response
AT+C5GNSSAIRDP=? +C5GNSSAIRDP: (list of supported <nssai_type>s),(list of
supported <plmn_id>s)
OK
Write Command Response
AT+C5GNSSAIRDP=<nssai_type>,<pl [+C5GNSSAIRDP: [<default_configured_nssai_length>,<
mn_id> default_configured_nssai>[,<rejected_nssai_3gpp_lengt
h>,<rejected_nssai_3gpp>[,<rejected_nssai_non3gpp_le
ngth>,<rejected_nssai_non3gpp>]]]
[+C5GNSSAIRDP: <plmn_id>[,<configured_nssai_lengt
h>,<configured_nssai>[,<allowed_nssai_3gpp_length>,<
allowed_nssai_3gpp>,<allowed_nssai_non3gpp_lengt
h>,<allowed_nssai_non3gpp>]]
[+C5GNSSAIRDP: <plmn_id>[,<configured_nssai_lengt
h>,<configured_nssai>[,<allowed_nssai_3gpp_length>,<
allowed_nssai_3gpp>,<allowed_nssai_non3gpp_lengt
h>,<allowed_nssai_non3gpp>]]
[...]]]]
OK
Maximum Response Time 300 ms
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 87 / 282

                                          5G Module Series
| Characteristics  |     |     | -               |     |     |     |     |     |     |
| ---------------- | --- | --- | --------------- | --- | --- | --- | --- | --- | --- |
| Reference        |     |     | 3GPP TS 27.007  |     |     |     |     |     |     |
Parameter
| <nssai_type>  |   Integer type. Type of NSSAI to be returned.  |                                              |     |     |     |     |     |     |     |
| ------------- | ---------------------------------------------- | -------------------------------------------- | --- | --- | --- | --- | --- | --- | --- |
|               |     1                                          | Return stored default configured NSSAI only  |     |     |     |     |     |     |     |
          2  Return stored default configured NSSAI and rejected NSSAI(s)
          3  Return stored default configured NSSAI, rejected NSSAI(s), and configured
|       |       | NSSAI(s)  |     |     |     |     |     |     |     |
| ----- | ----- | --------- | --- | --- | --- | --- | --- | --- | --- |
          4  Return stored default configured NSSAI, rejected NSSAI(s), configured
|       |       | NSSAI(s), and allowed NSSAI(s)  |     |     |     |     |     |     |     |
| ----- | ----- | ------------------------------- | --- | --- | --- | --- | --- | --- | --- |
<plmn_id>  String type. MCC and MNC of the PLMN to which the NSSAI
information applies. For the format and the encoding of the
MCC and MNC, see 3GPP TS 23.003. This parameter is not
|     |     |     | subject  | to  conventional  |     | character  | conversion  |     | as  per  |
| --- | --- | --- | -------- | ----------------- | --- | ---------- | ----------- | --- | -------- |
AT+CSCS.
<default_configured_nssai_length>  Integer type. Length in octets of the default configured NSSAI
stored in MT.
<default_configured_nssai>  String type in hexadecimal format. Depending on the form, the
string can be separated by dot(s), semicolon(s) and colon(s).
This parameter indicates the list of S-NSSAIs included in the
default configured NSSAI stored in MT for the PLMN. The
|     |     |     | <default_configured_nssai>  |            |     | is  coded  | as   | a  list    | of  <S-     |
| --- | --- | --- | --------------------------- | ---------- | --- | ---------- | ---- | ---------- | ----------- |
|     |     |     | NSSAI>s                     | separated  | by  | colon(s).  | See  | <S-NSSAI>  | in          |
3GPP 27.007 subclause 10.1.1. This parameter is not subject
to conventional character conversion as per AT+CSCS.
<rejected_nssai_3gpp_length>    Integer type. Length in octets of the rejected NSSAI
                  associated with 3GPP access stored in MT for the serving
|       |       |       | PLMN.  |     |     |     |     |     |     |
| ----- | ----- | ----- | ------ | --- | --- | --- | --- | --- | --- |
<rejected_nssai_3gpp>  String type in hexadecimal format. Depending on the form, the
string can be separated by dot(s), colon(s) and hash(es). This
parameter indicates the list of rejected S-NSSAIs associated
with 3GPP access stored in MT for the serving PLMN. The
|     |     |     | <rejected_nssai_3gpp>  |     | is  | coded  | as  a  list  | of  | rejected           |
| --- | --- | --- | ---------------------- | --- | --- | ------ | ------------ | --- | ------------------ |
<S-NSSAI>s separated by colon(s). For the format and the
|     |     |     | encoding  | of  <S-NSSAI>,  |     | see  also  | 3GPP  TS  | 23.003.  | This  |
| --- | --- | --- | --------- | --------------- | --- | ---------- | --------- | -------- | ----- |
parameter is not subject to conventional character conversion
as per AT+CSCS. Rejected S-NSSAI takes one of the forms:
                  sst#cause    only slice/service type (SST) and reject
|       |       |       |     |     cause are present   |     |     |     |     |     |
| ----- | ----- | ----- | --- | ----------------------- | --- | --- | --- | --- | --- |
                  sst.sd#cause   SST and slice differentiator (SD) and reject
                          cause are present where the cause is a
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                88 / 282

5G Module Series
cause value according to 3GPP TS 24.501
Table 9.11.3.46.1.
<rejected_nssai_non3gpp_length> Integer type. Length in octets of the rejected NSSAI associated
with non-3GPP access stored in MT for the serving PLMN.
<rejected_nssai_non3gpp> String type in hexadecimal format. Depending on the form, the
string can be separated by dot(s), colon(s) and hash(es). This
parameter indicates the list of rejected S-NSSAIs associated with
non-3GPP access stored in MT for the serving PLMN. The
<rejected_nssai_non3gpp> is coded as a list of rejected
<S-NSSAI>s separated by colon(s). For the format and the
encoding of <S-NSSAI>, see also 3GPP TS 23.003. This
parameter is not subject to conventional character conversion as
per AT+CSCS. The rejected S-NSSAI takes one of the following
forms:
sst#cause only slice/service type (SST) and reject cause are
present
sst.sd#cause SST and slice differentiator (SD) and reject cause
are present where cause is a cause value is
according to 3GPP TS 24.501 table 9.11.3.46.1.
<configured_nssai_length> Integer type. Length in octets of the configured NSSAI stored in
MT for the PLMN identified by <plmn_id>.
<configured_nssai> String type in hexadecimal format. Depending on the form, the
string can be separated by dot(s), semicolon(s) and colon(s). This
parameter indicates the list of configured S-NSSAIs stored in MT
for the PLMN identified by <plmn_id>. The <configured_nssai> is
coded as a list of <S-NSSAI>s separated by colon(s). See
<S-NSSAI> in 3GPP 27.007 subclause 10.1.1. This parameter is
not subject to conventional character conversion as per AT+CSCS.
<allowed_nssai_3gpp_length> Integer type. Length in octets of the allowed NSSAI associated with
3GPP access stored in MT for the PLMN identified by <plmn_id>.
<allowed_nssai_3gpp> String type in hexadecimal format. Depending on the form, the
string can be separated by dot(s), semicolon(s) and colon(s). This
parameter indicates the list of allowed S-NSSAIs associated with
3GPP access stored in MT for the PLMN identified by <plmn_id>.
The <allowed_nssai_3gpp> is coded as a list of <S-NSSAI>s
separated by colon(s). See <S-NSSAI> in 3GPP 27.007
subclause 10.1.1. This parameter is not subject to conventional
character conversion as per AT+CSCS.
<allowed_nssai_non3gpp_length> Integer type. Length in octets of the allowed NSSAI associated
with non-3GPP access stored in MT for the PLMN identified by
<plmn_id>.
<allowed_nssai_non3gpp> String type in hexadecimal format. Depending on the form, the
string can be separated by dot(s), semicolon(s) and colon(s).
This parameter indicates the list of allowed S-NSSAIs
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 89 / 282

5G Module Series
associated with non-3GPP access stored in MT for the PLMN
identified by <plmn_id>. The <allowed_nssai_non3gpp> is
coded as a list of <S-NSSAI>s separated by colon(s). See
<S-NSSAI> in 3GPP 27.007 subclause 10.1.1. This parameter
is not subject to conventional character conversion as per
AT+CSCS.
<S-NSSAI> String type in hexadecimal character format. Depending on the form, the string
can be separated by dot(s) and semicolon(s). This parameter is associated with
the PDU session for identifying a network slice in 5GS, see 3GPP TS 23.501 and
3GPP TS 24.501. For the format and the encoding of S-NSSAI, see also 3GPP
TS 23.003. This parameter is not subject to conventional character conversion as
per AT+CSCS. The parameter takes one of the following forms:
sst only slice/service type (SST) is present.
sst;mapped_sst SST and mapped configured SST are present
sst.sd SST and slice differentiator (SD) are present.
Sst.sd;mapped_sst SST, SD and mapped configured SST are present.
sst.sd;mapped_sst.mapped_sd SST, SD, mapped configured SST and
mapped configured SD are present.
5.9. AT+CSQ Signal Quality Report
This command indicates the received signal strength <RSSI> and the channel bit error rate <ber>. This
Test Command returns values supported by MT. This Execution Command returns the received signal
strength indication <RSSI> and the channel bit error rate <ber> from MT.
AT+CSQ Signal Quality Report
Test Command Response
AT+CSQ=? +CSQ: (list of supported <RSSI>s),(list of supported <ber>s)
OK
Execution Command Response
AT+CSQ +CSQ: <RSSI>,<ber>
OK
If there is any error:
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 90 / 282

                                          5G Module Series
Parameter
| <RSSI>   |   Integer type. Received signal strength indication.  |                            |
| -------- | ----------------------------------------------------- | -------------------------- |
|          |   0                                                   |   -113 dBm or less         |
|          |   1                                                   |   -111 dBm                 |
|          |   2–30                                                |   -109 dBm to -53 dBm      |
|          |   31                                                  |   -51 dBm or greater       |
|          |   99                                                  | Unknown or not detectable  |
| <ber>    |   Integer type. Channel bit error rate (in percent).  |                            |
        0–7        As RxQual values in the table in 3GPP TS 45.008 subclause 8.2.4
|                 |   99                                               |   Unknown or not detectable  |
| --------------- | -------------------------------------------------- | ---------------------------- |
| <err>           |   Error code. For more details, see Chapter 13.5.  |                              |
Example
AT+CSQ=?
+CSQ: (0-31,99),(0-7,99)

OK
AT+CSQ
+CSQ: 28,99
//The current signal strength indication is 28 and the channel bit error rate is unknown or not detectable.

OK

NOTE
1.  After using network-related commands such as AT+CCWA and AT+CCFC, it is recommended to
wait  for  3  s  before  entering  AT+CSQ  to  ensure  that  any  network  access  required  for  the
preceding command has been completed.
2.  This command only takes effect under WCDMA and LTE, and does not apply to 5G.

5.10. AT+QRSRP  Report RSRP

The command queries and reports the RSRP of the current service network.
AT+QRSRP  Report RSRP
Test Command    Response
AT+QRSRP=?  OK
Execution Command  Response
AT+QRSRP  +QRSRP: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                91 / 282

5G Module Series
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<PRX> Integer type. PRX path RSRP value. Range: -140 to -44 dBm.
<DRX> Integer type. DRX path RSRP value. Range: -140 to -44 dBm.
<RX2> Integer type. RX2 path RSRP value. Range: -140 to -44 dBm.
<RX3> Integer type. RX3 path RSRP value. Range: -140 to -44 dBm.
<sysmode> String type. It indicates the service mode in which the MT will report the RSRP.
LTE LTE mode
NR5G 5G mode
NOTE
1. This command is only supported in LTE and 5G.
2. If the queried <PRX>, <DRX>, <RX2> or <RX3> is -32768, it indicates that the RSRP value is
invalid.
3. This command is strongly related to the RF link and is generally only used for customer reference
and cannot be used as a sensitivity test. In addition, it is best to use it when measuring the speed,
the results are more accurate.
Example
AT+QRSRP //Query RSRP.
+QRSRP: -101,-105,-105,-99,LTE
OK
5.11. AT+QRSRQ Report RSRQ
The command queries and reports the RSRQ of the current service network.
AT+QRSRQ Report RSRQ
Test Command Response
AT+QRSRQ=? OK
Read Command Response
AT+QRSRQ +QRSRQ: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 92 / 282

5G Module Series
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<PRX> Integer type. PRX path RSRQ value. Range: -20 to -3 dB.
<DRX> Integer type. DRX path RSRQ value. Range: -20 to -3 dB.
<RX2> Integer type. RX2 path RSRQ value. Range: -20 to -3 dB.
<RX3> Integer type. RX3 path RSRQ value. Range: -20 to -3 dB.
<sysmode> String type. It indicates the service mode in which the MT will report the RSRQ.
LTE LTE mode
NR5G 5G mode
NOTE
1. This command is only supported in LTE and 5G.
2. If the queried <PRX>, <DRX>, <RX2> or <RX3> is -32768, it indicates that the RSRQ value is
invalid.
3. This command is strongly related to the RF link and is generally only used for customer reference
and cannot be used as a sensitivity test. In addition, it is best to use it when measuring the speed,
the results are more accurate.
Example
AT+QRSRQ //Query RSRQ.
+QRSRQ: -16,-19,-19,-15,LTE
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 93 / 282

5G Module Series
5.12. AT+QSINR Report SINR
The command queries and reports the SINR of the current service network.
AT+QSINR Report SINR
Test Command Response
AT+QSINR=? OK
Read Command Response
AT+QSINR? +QSINR: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>
OK
Execution Command Response
AT+QSINR +QSINR: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<PRX> Integer type. PRX path SINR value. Range: -20 to 30 dB in LTE, -23 to 40 dB in 5G.
<DRX> Integer type. DRX path SINR value. Range: -20 to 30 dB in LTE, -23 to 40 dB in 5G.
<RX2> Integer type. RX2 path SINR value. Range: -20 to 30 dB in LTE, -23 to 40 dB in 5G.
<RX3> Integer type. RX3 path SINR value. Range: -20 to 30 dB in LTE, -23 to 40 dB in 5G.
<sysmode> String type. It indicates the service mode in which the MT will report the SINR.
LTE LTE mode
NR5G 5G mode
NOTE
1. This command is only supported in LTE and 5G.
2. If the queried <PRX>, <DRX>, <RX2> or <RX3> is -32768, it indicates that the SINR value is
invalid.
3. This command is strongly related to the RF link and is generally only used for customer reference
and cannot be used as a sensitivity test. In addition, it is best to use it when measuring the speed,
the results are more accurate.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 94 / 282

                                          5G Module Series
Example
| AT+QSINR                    |       |     //Query SINR.  |
| --------------------------- | ----- | ------------------ |
+QSINR: -3,-7,-1,-2,LTE

OK

5.13. AT+CPOL  Preferred Operator List

This command edits and queries the list of preferred operators.
AT+CPOL  Preferred Operator List
| Test Command  |     | Response  |
| ------------- | --- | --------- |
AT+CPOL=?  +CPOL:  (list  of  supported  <index>s),(list  of  supported
<format>s)

OK
| Read Command  |     | Response  |
| ------------- | --- | --------- |
Query the list of preferred operators:  +CPOL: <index>,<format>,<oper>[,<GSM>,<GSM_compac
| AT+CPOL?  |     | t>,<UTRAN>,<E-UTRAN>,<NG-RAN>]  |
| --------- | --- | ------------------------------- |
[…]

OK
| Write Command                          |     | Response  |
| -------------------------------------- | --- | --------- |
| Edit the list of preferred operators:  |     | OK        |
| AT+CPOL=<index>[,<format>[,<ope        |     | Or        |
| r>[<GSM>,<GSM_compact>,<UTRA           |     | ERROR     |
| N>,<E-UTRAN>,<NG-RAN>]]]               |     |           |
If there is any error related to MT functionality:
+CME ERROR: <err>

If <index> is given but <oper> is omitted, the entry is deleted.
| Maximum Response Time  |     | 300 ms          |
| ---------------------- | --- | --------------- |
| Characteristics        |     | -               |
| Reference              |     | 3GPP TS 27.007  |
Parameter
<index>         Integer type. Order number of the operator in the (U)SIM preferred operator list.
<format>    Integer type. Format of operator name.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                95 / 282

                                          5G Module Series
|       |   0    | Long format alphanumeric <oper>   |
| ----- | ------ | --------------------------------- |
|       |   1    | Short format alphanumeric <oper>  |
|       |   2    | Numeric <oper>                    |
<oper>         String type. Operation name. <format> indicates if the format is alphanumeric or
|             |   numeric (see AT+COPS).                          |                |
| ----------- | ------------------------------------------------- | -------------- |
| <GSM>       |   Integer type. GSM access technology selection.  |                |
|             |   0                                               | Not selected   |
|             |   1                                               | Selected       |
<GSM_compact>  Integer type. GSM compact access technology selection.
|     |     0  | Not selected  |
| --- | ------ | ------------- |
|     |     1  | Selected      |
<UTRAN>        Integer type. UTRAN access technology selection.
|     |     0  | Not selected  |
| --- | ------ | ------------- |
|     |     1  | Selected      |
<E-UTRAN>      Integer type. E-UTRAN access technology selection.
|     |     0  | Not selected  |
| --- | ------ | ------------- |
1  Selected
<NG-RAN>      Integer type. NG-RAN access technology selection.
0  Not selected
1  Selected
<err>        Error code. For more details, see Chapter 13.5.

NOTE
The access technology selection parameters <GSM>, <GSM_compact>, <UTRAN> and <E-UTRAN>
are required for (U)SIM card or UICC’s containing PLMN selector with access technology.

5.14. AT+COPN  Read Operator Names

This command returns the list of supported operator names from MT. Each operator code <numericn>
that has an alphanumeric equivalent <alphan> in the MT memory is returned.
AT+COPN  Read Operator Names
Test Command  Response
AT+COPN=?  OK
Execution Command  Response
AT+COPN  +COPN: <numeric1>,<alpha1>
[+COPN: <numeric2>,<alpha2>
[…]]

OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                96 / 282

5G Module Series
If there is error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time Determined by the number of operator names.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<numeric> String type. Operator name in numeric format (see AT+COPS).
<alpha> String type. Operator name in long alphanumeric format (see AT+COPS).
<err> Error code. For more details, see Chapter 13.5.
5.15. AT+CTZU Automatic Time Zone Update
This command enables/disables automatic time zone update via NITZ.
AT+CTZU Automatic Time Zone Update
Test Command Response
AT+CTZU=? +CTZU: (list of supported <onoff>s)
OK
Write Command Response
AT+CTZU=<onoff> OK
Or
ERROR
Read Command Response
AT+CTZU? +CTZU: <onoff>
OK
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 97 / 282

5G Module Series
Parameter
<onoff> Integer type. Enable or disable automatic time zone update.
0 Disable
1 Enable
NOTE
Executing AT+CTZU=<onoff> writes data to NVM. Please proceed with caution.
Example
AT+CTZU? //Read command.
+CTZU: 0
OK
AT+CTZU=? //Test command.
+CTZU: (0,1)
OK
AT+CTZU=1 //Enable automatic time zone update.
OK
AT+CTZU?
+CTZU: 1
OK
5.16. AT+CTZR Time Zone Reporting
This command controls time zone change event reporting. If reporting is enabled, MT returns URC
+CTZV: <tz> or +CTZE: <tz>,<dst>,[<time>] whenever the time zone is changed.
AT+CTZR Time Zone Reporting
Test Command Response
AT+CTZR=? +CTZR: (list of supported <reporting>s)
OK
Read Command Response
AT+CTZR? +CTZR: <reporting>
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 98 / 282

5G Module Series
Write Command Response
AT+CTZR=<reporting> OK
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Reference 3GPP TS 27.007
Parameter
<reporting> Integer type. Disable or enable time zone reporting.
0 Disable
1 Enable time zone change event reporting by URC +CTZV: <tz>
2 Enable extended time zone change event reporting by URC +CTZE:
<tz>,<dst>,[<time>]
<tz> String type. Sum of local time zone and daylight saving time (difference between local
time and GMT is expressed in quarter(s) of an hour). Format: "±zz", where "zz" is a fixed
width, two-digit integer with the range -48 to +56. To maintain a fixed width, numbers in
range -9 to +9 are expressed with a leading zero, e.g. "-09", "+00" and "+09".
<dst> Integer type. It indicates whether <tz> includes daylight saving time adjustment.
0 <tz> does not include adjustment for daylight saving time
1 <tz> includes +1 hour adjustment (equivalent to 4 quarters in <tz>) for daylight
saving time
2 <tz> includes +2 hours adjustment (equivalent to 8 quarters in <tz>) for daylight
saving time
<time> String type. Local time. Format: "YYYY/MM/DD,hh:mm:ss", expressed as integers
representing year (YYYY), month (MM), date (DD), hour (hh), minute (mm) and second
(ss). This parameter can be provided by the network when delivering time zone
information and will be presented in URC of extended time zone change event reporting
if provided by the network.
NOTE
Executing AT+CTZR=<reporting> writes data to NVM. Please proceed with caution.
Example
AT+CTZR=2
OK
AT+CTZR?
+CTZR: 2
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 99 / 282

5G Module Series
OK
+CTZE: "+32",0,"2018/03/23,06:51:13" //Extended time zone and local time reporting by URC.
5.17. AT+QLTS Obtain Latest Time Synchronized Through Network
The Execution Command returns the latest time synchronized through the network.
AT+QLTS Obtain Latest Time Synchronized Through Network
Test Command Response
AT+QLTS=? +QLTS: (list of supported <mode>s)
OK
Write Command Response
AT+QLTS=<mode> +QLTS: <time>,<ds>
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Execution Command Response
AT+QLTS +QLTS: <time>,<ds>
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<mode> Integer type. Query network time mode.
0 Query the latest time that has been synchronized through network
1 Query the current GMT time calculated from the latest time that has been
synchronized through network
2 Query the current local time calculated from the latest time that has been
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 100 / 282

5G Module Series
synchronized through network
<time> String type. Format is "yyyy/MM/dd,hh:mm:ss±zz", where characters represent year
month, day, hour, minute, second and time zone (indicating the difference, expressed in
quarter(s) of an hour, between local time and GMT; range: -48 to +48). E.g., 6th of May
2004, 22:10:00 GMT+2 equals "2004/05/06,22:10:00+08".
<ds> Integer type. Daylight saving time.
0 No adjustment
1 Plus one hour
2 Plus two hours
<err> Error code. For more details, see Chapter 13.5.
NOTE
If the time has not been synchronized through network, the command returns +QLTS: "".
Example
AT+QLTS=? //Query supported network time modes.
+QLTS: (0-2)
OK
AT+QLTS //Query the latest time synchronized through network.
+QLTS: "2017/01/13,03:40:48+32",0
OK
AT+QLTS=0
//Query the latest time synchronized through network. It offers the same
function as the Execution Command AT+QLTS.
+QLTS: "2017/01/13,03:40:48+32",0
OK
AT+QLTS=1 //Query the current GMT time calculated from the latest time that has been
synchronized through network.
+QLTS: "2017/01/13,03:41:22+32",0
OK
AT+QLTS=2 //Query the current local time calculated from the latest time that has been
synchronized through network.
+QLTS: "2017/01/13,11:41:23+32",0
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 101 / 282

5G Module Series
5.18. AT+QNWINFO Query Network Information
This command queries network information such as the selected access technology, the operator and
the selected band.
AT+QNWINFO Query Network Information
Test Command Response
AT+QNWINFO=? OK
Execution Command Response
AT+QNWINFO +QNWINFO: <AcT>,<oper>,<band>,<channel>
[+QNWINFO: <AcT>,<oper>,<band>,<channel>]
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<AcT> String type. Selected access technology.
"NONE"
"WCDMA"
"TDD LTE"
"FDD LTE"
"TDD NR5G"
"FDD NR5G"
<oper> String type. Operator name in numeric format without double quotes.
<band> String type. Selected band.
"WCDMA_I_2100"
"WCDMA_II_1900"
"WCDMA_III_1800"
"WCDMA_IV_1700_US"
"WCDMA_V_850"
"WCDMA_VI_800"
"WCDMA_VII_2600"
"WCDMA_VIII_900"
"WCDMA_IX_1700_JAPAN"
"WCDMA_XI_1500"
"WCDMA_XIX_850_JAPAN"
"LTE BAND 1"–"LTE BAND 43"
"LTE BAND 46"–"LTE BAND49"
"LTE BAND 66"–"LTE BAND 68"
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 102 / 282

                                          5G Module Series
|            | "LTE BAND 71"                    |     |
| ---------- | -------------------------------- | --- |
|            | "LTE BAND 125"–"LTE BAND 127"    |     |
|            | "LTE BAND 250"                   |     |
|            | "LTE BAND 252"                   |     |
|            | "LTE BAND 255"                   |     |
|            | "NR5G BAND 1"–"NR5G BAND 3"      |     |
|            | "NR5G BAND 5"                    |     |
|            | "NR5G BAND 7"–"NR5G BAND 8"      |     |
|            | "NR5G BAND 12"                   |     |
|            | "NR5G BAND 14"                   |     |
|            | "NR5G BAND 20"                   |     |
|            | "NR5G BAND 25"                   |     |
|            | "NR5G BAND 28"                   |     |
|            | "NR5G BAND 34"                   |     |
|            | "NR5G BAND 38"–"NR5G BAND 41"    |     |
|            | "NR5G BAND 48"                   |     |
|            | "NR5G BAND 50"–"NR5G BAND 51"    |     |
|            | "NR5G BAND 65"–"NR5G BAND 66"    |     |
|            | "NR5G BAND 70"–"NR5G BAND 71"    |     |
|            | "NR5G BAND 74"–"NR5G BAND 86"    |     |
|            | "NR5G BAND 257"–"NR5G BAND 261"  |     |
| <channel>  | Integer type. Channel ID.        |     |

NOTE
If the device has not been registered on network, the command returns +QNWINFO: No Service. For
5G NSA, it returns both LTE and 5G information.
Example
AT+QNWINFO=?
OK
AT+QNWINFO
+QNWINFO: "FDD LTE",46001,"LTE BAND 3",1650

OK

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                103 / 282

5G Module Series
5.19. AT+QSPN Query Service Provider Name
This command queries the service provider name.
AT+QSPN Query Service Provider Name
Test Command Response
AT+QSPN=? OK
Execution Command Response
AT+QSPN +QSPN: <FNN>,<SNN>,<SPN>,<alphabet>,<RPLMN>
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Parameter
<FNN> String type. Full name of network.
<SNN> String type. Shortened name of network.
<SPN> String type. Service provider name.
<alphabet> Integer type. Alphabet of full and shortened network name.
0 GSM 7-bit default alphabet
1 UCS2
<RPLMN> String type. Registered PLMN.
NOTE
1. If <alphabet> is 0, <FNN> and <SNN> are shown in GSM 7-bit default alphabet string.
2. If <alphabet> is 1, <FNN> and <SNN> are shown in UCS2 hexadecimal string.
3. If network is not registered, AT+QSPN returns OK.
Example
AT+QSPN //Query the service provider name.
+QSPN: "CHN-UNICOM","UNICOM","",0,"46001"
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 104 / 282

5G Module Series
5.20. AT+QENG Query Primary Serving Cell and Neighbour Cell
Information
This command obtains the network information, such as serving cell and neighbour cell.
AT+QENG Query Primary Serving Cell and Neighbour Cell Information
Test Command Response
AT+QENG=? +QENG: (list of supported <cell_type>s)
OK
Write Command Response
Query the serving cell information In SA mode:
AT+QENG="servingcell" +QENG: "servingcell",<state>,"NR5G-SA",<duplex_mod
e>,<MCC>,<MNC>,<cellID>,<PCID>,<TAC>,<ARFCN>,<ba
nd>,<NR_DL_bandwidth>,<RSRP>,<RSRQ>,<SINR>,<sc
s>,<srxlev>
OK
In EN-DC mode:
+QENG: "servingcell",<state>
+QENG: "LTE",<is_tdd>,<MCC>,<MNC>,<cellID>,<PCID>,
<earfcn>,<freq_band_ind>,<UL_bandwidth>,<DL_bandwi
dth>,<TAC>,<RSRP>,<RSRQ>,<RSSI>,<SINR>,<CQI>,<tx
_power>,<srxlev>
+QENG: "NR5G-NSA",<MCC>,<MNC>,<PCID>,<RSRP>,<
SINR>,<RSRQ>,<ARFCN>,<band>,<NR_DL_bandwidth>,
<scs>
OK
In LTE mode:
+QENG: "servingcell",<state>,"LTE",<is_tdd>,<MCC>,<M
NC>,<cellID>,<PCID>,<earfcn>,<freq_band_ind>,<UL_ba
ndwidth>,<DL_bandwidth>,<TAC>,<RSRP>,<RSRQ>,<RS
SI>,<SINR>,<CQI>,<tx_power>,<srxlev>
OK
In WCDMA mode:
+QENG: "servingcell",<state>,"WCDMA",<MCC>,<MNC>,
<LAC>,<cellID>,<uarfcn>,<PSC>,<RAC>,<RSCP>,<ecio>,
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 105 / 282

5G Module Series
<phych>,<SF>,<slot>,<speech_code>,<comMod>
OK
If there is any error:
ERROR
Write Command Response
Query the information of neighbour cells In LTE mode:
AT+QENG="neighbourcell" [+QENG: "neighbourcell intra","LTE",<earfcn>,<PCID>,<
RSRQ>,<RSRP>,<RSSI>,<SINR>,<srxlev>,<cell_resel_pri
ority>,<s_non_intra_search>,<thresh_serving_low>,<s_i
ntra_search>]
[…]
[+QENG: "neighbourcell inter","LTE",<earfcn>,<PCID>,<
RSRQ>,<RSRP>,<RSSI>,<SINR>,<srxlev>,<cell_resel_pri
ority>,<threshX_low>,<threshX_high>]
[…]
[+QENG:"neighbourcell","WCDMA",<uarfcn>,<cell_resel
_priority>,<thresh_Xhigh>,<thresh_Xlow>,<PSC>,<RSC
P><ecno>,<srxlev>]
[…]
OK
In WCDMA mode:
[+QENG:"neighbourcell","WCDMA",<uarfcn>,<srxqual>,
<PSC>,<RSCP>,<ecno>,<set>,<rank>,<srxlev>]
[…]
[+QENG: "neighbourcell","LTE",<earfcn>,<PCID>,<RSR
P>,<RSRQ>,<srxlev>]
[…]
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<cell_type> String type. Information of different cells.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 106 / 282

5G Module Series
"servingcell" Information of 3G/4G/5G serving cells
"neighbourcell" Information of 3G/4G neighbor cells
<state> String type. UE state.
"SEARCH" UE is searching but cannot (yet) find a suitable 3G/4G/5G cell.
"LIMSRV" UE is camping on a cell but has not registered on the network.
"NOCONN" UE is camping on a cell and has registered on the network,
and it is in idle mode.
"CONNECT" UE is camping on a cell and has registered on the network,
and a call is in progress.
<duplex_mode> String type. 5G SA network mode.
"TDD"
"FDD"
<MCC> 16-bit unsigned integer. Mobile country code (first part of the PLMN code).
<MNC> 16-bit unsigned integer. Mobile network code (second part of the PLMN code).
<cellID> Integer type. Cell ID. 28-bit (UMTS, LTE) or 36-bit (5G) cell ID. Range:
0–0xFFFFFFFFF.
<PCID> Integer type. Physical cell ID.
<TAC> String type. Two-byte tracking area code for LTE or three-byte tracking area
code for 5G SA in hexadecimal format without double quotes (see
3GPP 23.003 Section 19.4.2.3).
<ARFCN> Integer type. SA-ARFCN of the scanned cell.
<band> 32-bit unsigned integer. Frequency band in 5G SA network mode.
<NR_DL_bandwidth> Integer type. DL bandwidth. It is only valid in RRC connected state.
0 5 MHz
1 10 MHz
2 15 MHz
3 20 MHz
4 25 MHz
5 30 MHz
6 40 MHz
7 50 MHz
8 60 MHz
9 70 MHZ
10 80 MHz
11 90 MHz
12 100 MHz
13 200 MHz
14 400 MHz
15 35 MHz
16 45 MHz
<RSRP> 16-bit signed integer.
- In LTE mode:
It indicates the signal of LTE Reference Signal Received Power (see
3GPP 36.214). Range: -140 to -44 dBm. A value closer to -44 indicates a
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 107 / 282

5G Module Series
stronger signal, whereas the value closer to -140 indicates a weaker signal.
- In 5G mode:
It indicates the signal of 5G Reference Signal Received Power. Range:
-140 to -44 dBm. A value closer to -44 indicates a stronger signal, whereas
the value closer to -140 indicates a weaker signal.
<RSRQ> 16-bit signed integer.
- In LTE mode:
It indicates the signal of current LTE Reference Signal Received Quality (see
3GPP 36.214). Range: -20 to -3 dB. A value closer to -3 indicates a stronger
signal, whereas the value closer to -20 indicates a weaker signal.
- In 5G mode:
It indicates the signal of current 5G Reference Signal Received Quality.
Range: -20 to -3 dB. A value closer to -3 indicates a stronger signal, whereas
the value closer to -20 indicates a weaker signal.
<SINR> 16-bit signed integer.
- In LTE mode:
It indicates LTE Signal-to-Interface plus Noise Ratio. The conversion formula
for actual SINR is Y = (1/5) × X × 10 - 20 (X is the <SINR> value queried by
AT+QENG and Y is the actual value of LTE SINR after calculating with the
formula). Range: -20 to 30 dB.
- In 5G mode:
It indicates the signal of 5G Signal-to-Interface plus Noise Ratio. Range: -23
to 40 dB.
<scs> Integer type. NR subcarrier space.
0 15 kHz
1 30 kHz
2 60 kHz
3 120 kHz
4 240 kHz
<srxlev> Integer type. Reception level value for the selected base station in dB (see
3GPP 25.304)..
<is_tdd> String type. LTE network mode.
"TDD"
"FDD"
<earfcn> Integer type. E-UTRA-ARFCN of the scanned cell.
<freq_band_ind> Integer type. E-UTRA frequency band (see 3GPP 36.101).
<UL_bandwidth> Integer type. UL bandwidth.
0 1.4 MHz
1 3 MHz
2 5 MHz
3 10 MHz
4 15 MHz
5 20 MHz
<DL_bandwidth> Integer type. DL bandwidth.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 108 / 282

5G Module Series
0 1.4 MHz
1 3 MHz
2 5 MHz
3 10 MHz
4 15 MHz
5 20 MHz
<RSSI> Integer type. LTE Received Signal Strength Indication.
<CQI> Integer type. Channel Quality Indication. Range: 1–30.
<tx_power> Integer type. TX power value in 1/10 dBm. It is the maximum of all UL channel
TX power. <tx_power> is only meaningful when the device is in traffic.
<LAC> Integer type. Location area code. Range: 0–65535. It determines the two-byte
location area code in hexadecimal format (e.g. 00C1 equals 193 in decimal) of
the scanned cell.
<uarfcn> Integer type. UTRA-ARFCN of scanned cell.
<PSC> Integer type. Primary scrambling code of scanned cell.
<RAC> Integer type. Routing Area Code. Range: 0–255.
<RSCP> Integer type. Received Signal Code Power level of scanned cell.
<ecio> Integer type. Carrier to noise ratio in dB = measured Ec/Io value in dB.
<phych> Integer type. Physical channel.
0 DPCH
1 FDPCH
<SF> Integer type. Spreading factor.
0 SF_4
1 SF_8
2 SF_16
3 SF_32
4 SF_64
5 SF_128
6 SF_256
7 SF_512
8 UNKNOWN
<slot> Integer type.
0–16 Slot format for DPCH
0–9 Slot format for FDPCH
<speech_code> Integer type. Destination number on which call is to be deflected.
<comMod> Integer type. Number format. Compress mode.
0 Not support compress mode
1 Support compress mode
<cell_resel_priority> Integer type. Cell reselection priority. Range: 0–7.
<s_non_intra_search> Integer type. Threshold to control non-intra frequency search.
<thresh_serving_low> Integer type. It specifies the suitable reception level threshold (in dB) used by
the UE on the serving cell when reselecting towards a lower priority
RAT/frequency.
<s_intra_search> Integer type. Cell selection parameter for the intra frequency cell.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 109 / 282

5G Module Series
<threshX_low> Integer type. To be considered for re-selection. Suitable receive level value of
an evaluated lower priority cell must be greater than this value.
<threshX_high> Integer type. To be considered for re-selection. Suitable receive level value of
an evaluated higher priority cell must be greater than this value.
<thresh_Xhigh> Integer type. Reselection threshold for high priority layers.
<thresh_Xlow> Integer type. Reselection threshold for low priority layers.
<srxqual> Integer type. Receiver automatic gain control on the camped frequency.
<ecno> Integer type. Ratio of the received energy per PN chip to the total received
power spectral density (see 3GPP TS 25.133).
<set> Integer type. 3G neighbor cell set.
1 Active set
2 Synchronous neighbor set
3 Asynchronous neighbor set
<rank> Integer type. Rank of this cell as neighbor for inter-RAT cell reselection.
NOTE
"-" or - indicates the parameter is invalid under current condition.
Example
AT+QENG="servingcell"
+QENG: "servingcell","NOCONN","LTE","FDD",460,01,5F1EA15,12,1650,3,5,5,DE10,-100,-12,-68,11,
-,-,27
OK
AT+QENG="servingcell"
+QENG: "servingcell","NOCONN"
+QENG: "LTE","FDD",460,01,5F1EA15,12,1650,3,5,5,DE10,-99,-12,-67,11,9,230,-
+QENG:"NR5G-NSA",460,01,747,-71,13,-11,627264,78,12,1
OK
AT+QENG="servingcell"
+QENG: "servingcell","NOCONN","NR5G-SA","TDD",460,01,9013B004,299,690E0F,633984,78,12,-1
07,-13,2,1,-
OK
AT+QENG="neighbourcell"
+QENG: "neighbourcell intra","LTE",38950,276,-3,-88,-65,0,37,7,16,6,44
+QENG: "neighbourcell inter","LTE",39148,-,-,-,-,-,37,0,30,7
+QENG: "neighbourcell inter","LTE",37900,-,-,-,-,-,0,0,30,6
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 110 / 282

5G Module Series
5.21. AT+QCAINFO Query Carrier Aggregation Parameters
This command queries carrier aggregation parameters.
AT+QCAINFO Query Carrier Aggregation Parameters
Test Command Response
AT+QCAINFO=? +QCAINFO: (list of supported <5G_signal_ext>)
OK
Read Command Response
AT+QCAINFO? +QCAINFO: <5G_signal_ext>
OK
Write Command Response
AT+QCAINFO=<5G_signal_ext> OK
Or
ERROR
Execution Command In LTE mode:
AT+QCAINFO +QCAINFO: "PCC",<freq>,<bandwidth>,<band>,<pcell_sta
te>,<PCID>,<RSRP>,<RSRQ>,<RSSI>,<RSSNR>
[+QCAINFO: "SCC",<freq>,<bandwidth>,<band>,<scell_st
ate>,<PCID>,<RSRP>,<RSRQ>,<RSSI>,<RSSNR><UL_con
figured>,<UL_bandwidth>,<UL_EARFCN>]
[…]
OK
In EN-DC mode:
+QCAINFO: "PCC",<freq>,<bandwidth>,<band>,<pcell_sta
te>,<PCID>,<RSRP>,<RSRQ>,<RSSI>,<RSSNR>
[+QCAINFO: "SCC",<freq>,<bandwidth>,<band>,<scell_st
ate>,<PCID>,<RSRP>,<RSRQ>,<RSSI>,<RSSNR><UL_con
figured>,<UL_bandwidth>,<UL_EARFCN>]
[…]
[+QCAINFO: "SCC",<freq>,<NR_DL_bandwidth>,<NR_ban
d>,<PCID>][,<NR_RSRP>,<NR_RSRQ>[,<NR_SNR>]]
[+QCAINFO: "SCC",<freq>,<NR_DL_bandwidth>,<NR_ban
d>,<scell_state>,<PCID>,<UL_configured>,<NR_UL_band
width>,<UL_ARFCN>[,<NR_RSRP>,<NR_RSRQ>[,<NR_SN
R>]]
[…]
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 111 / 282

                                          5G Module Series

In SA mode:
+QCAINFO: "PCC",<freq>,<NR_DL_bandwidth>,<NR_ban
d>,<PCID>[,<NR_RSRP>,<NR_RSRQ>[,<NR_SNR>]]
[+QCAINFO: "SCC",<freq>,<NR_DL_bandwidth>,<NR_ban
d>,<cell_state>,<PCID>,<UL_configured>,<NR_UL_bandw
idth>,<UL_ARFCN>[,<NR_RSRP>,<NR_RSRQ>[,<NR_SN
R>]]
[…]

OK

If there is any error:
ERROR
Maximum Response Time  300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<5G_signal_ext>  Integer type. Hide or show extension parameters <NR_RSRP>, <NR_RSRQ> and
<NR_SNR>
|                    |   0                                      | Hide     |
| ------------------ | ---------------------------------------- | -------- |
|                    |     1                                    | Show     |
| <freq>             |   Integer type. EARFCN.                  |          |
| <bandwidth>        | Integer type. Bandwidth.                 |          |
|                    |   6                                      | 1.4 MHz  |
|                    |   15                                     | 3 MHz    |
|                    |   25                                     | 5 MHz    |
|                    |   50                                     | 10 MHz   |
|                    |   75                                     | 15 MHz   |
|                    |   100                                    | 20 MHz   |
| <band>             |   String type. LTE DL band information.  |          |
|                    |   "LTE BAND 1"                           |          |
|                    |   "LTE BAND 2"                           |          |
|                    |   "LTE BAND 3"                           |          |
|                    |   …                                      |          |
|                    |   "LTE BAND 66"                          |          |
<pcell_state>     Integer type. Primary cell state.
|       |   0  Not registered, not searching  |     |
| ----- | ----------------------------------- | --- |
|       |   1  Registered on home network     |     |
|       |   2  Not registered, searching      |     |
|       |   3  Registration denied            |     |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                112 / 282

                                          5G Module Series
|       |   4  Unknow registration state  |     |
| ----- | ------------------------------- | --- |
5  Registered on roaming network
| <PCID>   |   Integer type. Physical Cell ID.  |     |
| -------- | ---------------------------------- | --- |
<RSRP>     Integer type. Reference Signal Received Power (see 3GPP 36.214)
<RSRQ>    Integer type. Reference Signal Received Quality (see 3GPP 36.214)
| <RSSI>   |   Integer type. Received Signal Strength Indication.  |     |
| -------- | ----------------------------------------------------- | --- |
<RSSNR>    Integer type. Logarithmic value of RSSNR. Range: -10 to +30 dB.
| <scell_state>  | Integer type. Secondary cell state.  |     |
| -------------- | ------------------------------------ | --- |
|                |   0  Deconfigured                    |     |
1  Configuration deactivated
|     |   2  Configuration activated  |     |
| --- | ----------------------------- | --- |
<UL_configured>    Integer type. Whether the UL of secondary cell is configured by network.
|     |       | 0  Not configured  |
| --- | ----- | ------------------ |
|     |       | 1  Configured      |
<UL_bandwidth>    Integer type. UL bandwidth. "-" will be displayed if <UL_configured>=0.
|     |       | 6    1.4 MHz   |
| --- | ----- | -------------- |
|     |       | 15    3 MHz    |
|     |       | 25    5 MHz    |
|     |       | 50    10 MHz   |
|     |       | 75    15 MHz   |
|     |       | 100   20 MHz   |
<UL_EARFCN>      Integer type. UL EARFCN. "-" will be displayed if <UL_configured>=0.
| <NR_DL_bandwidth>  |     | Integer type. NR downlink bandwidth.  |
| ------------------ | --- | ------------------------------------- |
0     5 MHz
1     10 MHz
2     15 MHz
3     20 MHz
4     25 MHz
5     30 MHz
6     40 MHz
7     50 MHz
8     60 MHz
9     70 MHz
10     80 MHz
11     90 MHz
12     100 MHz
13     200 MHz
14     400 MHz
15     35 MHz
16     45 MHz
| <NR_band>  |       | String Type. NR DL band information.  |
| ---------- | ----- | ------------------------------------- |
|            |       | "NR5G BAND 1"                         |
|            |       | "NR5G BAND 2"                         |
|            |       | "NR5G BAND 3”                         |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                113 / 282

5G Module Series
…
"NR5G BAND 261"
<NR_RSRP> Integer type. NR5G Reference Signal Received Power. Range: -140 to -44;
Unit: dBm. The closer to -44, the better the signal is. The closer to -140,
the worse the signal is.
<NR_RSRQ> Integer type. Current NR5G Reference Signal Received Quality. Range: -43
to 20; Unit: dB. The closer to 20, the better the signal is. The closer to -43,
the worse the signal is.
<NR_SNR> Integer type. Current NR5G SNR. Range: -2300 to 4000. The actual value of
NR SNR is calculated via the formula:
NR SNR = <NR_SNR> / 100
Range of NR SNR: -23 to 40; Unit: dB.
<NR_UL_bandwidth> Integer type. "-" will be displayed if <UL_configured>=0. The value of
<NR_UL_bandwidth> is the same as that of <NR_DL_bandwidth>.
<UL_ARFCN> Integer type. UL_ARFCN. "-" will be displayed if <UL_configured> is 0.
NOTE
This command is valid only after the module registers on network.
Example
AT+QCAINFO
+QCAINFO: "PCC",300,100,"LTE BAND 1",1,23,-66,-12,-34,30
+QCAINFO: "SCC",1575,100,"LTE BAND 3",2,43,-64,-7,-24,30,0,-,-
OK
5.22. AT+QNETRC Get the Cause of Network Rejection
This command gets the cause of network rejection. This Write Command sets whether to present URC
and controls the presentation of the URC +QNETRC: "emm_cause",<emm_reject_cause> when
<mode> & 0x01 = 1 and the module receives a rejection code issued by the network during LTE network
registration, the URC +QNETRC: "esm_cause",<esm_reject_cause> when <mode> & 0x02 = 2 and
the module receives a rejection code issued by the network during LTE session management process, or
the URC +QNETRC: "5gmm_cause",<5gmm_reject_cause> when <mode> & 0x4 = 4 and the module
receives a rejection code issued by the network during 5G network registration.
AT+QNETRC Get the Cause of Network Rejection
Read Command Response
AT+QNETRC? +QNETRC: "emm_cause",<emm_reject_cause>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 114 / 282

                                          5G Module Series
+QNETRC: "esm_cause",<esm_reject_cause>
+QNETRC: "5gmm_cause",<5gmm_reject_cause>

OK
Write Command  Response
AT+QNETRC=<mode>  OK
Or
ERROR
Execution Command  Response
AT+QNETRC  +QNETRC: <mode>

OK
Characteristics  -
Parameter
<mode>         Integer type. Determines the output type of URC sentences by bitwise OR.
|                     |       | 0    No URC report                |
| ------------------- | ----- | --------------------------------- |
|                     |       | 1    EMM URC                      |
|                     |       | 2    ESM URC                      |
|                     |       | 4    5GMM URC                     |
| <emm_reject_cause>  |       | Integer type. EMM reject cause.   |
|                     |       | 0    No cause                     |
|                     |       | 2    IMSI unknown in HSS          |
|                     |       | 3    Illegal UE                   |
|                     |       | 5    IMEI not accepted            |
|                     |       | 6    Illegal ME                   |
|                     |       | 7    EPS services not allowed     |
            8    EPS services and non-EPS services not allowed
            9    UE identity cannot be derived by the network
|       |       | 10    Implicitly detached                        |
| ----- | ----- | ------------------------------------------------ |
|       |       | 11    PLMN not allowed                           |
|       |       | 12    Tracking Area not allowed                  |
|       |       | 13    Roaming not allowed in this tracking area  |
|       |       | 14    EPS services not allowed in this PLMN      |
|       |       | 15    No Suitable Cells in tracking area         |
|       |       | 16    MSC temporarily not reachable              |
|       |       | 17    Network failure                            |
|       |       | 18    CS domain not available                    |
|       |       | 19    ESM failure                                |
|       |       | 20    MAC failure                                |
|       |       | 21    Synch failure                              |
|       |       | 22    Congestion                                 |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                115 / 282

                                          5G Module Series
|       |       | 23    UE security capabilities mismatch    |
| ----- | ----- | ------------------------------------------ |
|       |       | 24    Security mode rejected, unspecified  |
|       |       | 25    Not authorized for this CSG          |
|       |       | 26    Non-EPS authentication unacceptable  |
|       |       | 31    Redirection to 5GCN required         |
            35    Requested service option not authorized in this PLMN
|       |       | 39    CS service temporarily not available  |
| ----- | ----- | ------------------------------------------- |
|       |       | 40    No EPS bearer context activated       |
|       |       | 42    Severe network failure                |
|       |       | 95    Semantically incorrect message        |
|       |       | 96    Invalid mandatory information         |
            97    Message type non-existent or not implemented
            98    Message type not compatible with the protocol state
            99    Information element non-existent or not implemented
|       |       | 100   Conditional IE error  |
| ----- | ----- | --------------------------- |
            101   Message not compatible with the protocol state
|                      |       | 111   Protocol error, unspecified               |
| -------------------- | ----- | ----------------------------------------------- |
| <esm_reject_cause>   |       | Integer type. ESM reject cause.                 |
|                      |       | 0    No cause                                   |
|                      |       | 8    Operator Determined Barring                |
|                      |       | 26    Insufficient resources                    |
|                      |       | 27    Missing or unknown APN                    |
|                      |       | 28    Unknown PDN type                          |
|                      |       | 29    User authentication failed                |
|                      |       | 30    Request rejected by Serving GW or PDN GW  |
|                      |       | 31    Request rejected, unspecified             |
|                      |       | 32    Service option not supported              |
|                      |       | 33    Requested service option not subscribed   |
|                      |       | 34    Service option temporarily out of order   |
|                      |       | 35    PTI already in use                        |
|                      |       | 36    Regular deactivation                      |
|                      |       | 37    EPS QoS not accepted                      |
|                      |       | 38    Network failure                           |
|                      |       | 39    Reactivation requested                    |
|                      |       | 41    Semantic error in the TFT operation       |
|                      |       | 42    Syntactical error in the TFT operation    |
|                      |       | 43    Invalid EPS bearer identity               |
|                      |       | 44    Semantic errors in packet filter(s)       |
|                      |       | 45    Syntactical errors in packet filter(s)    |
|                      |       | 46    Unused (see NOTE 2)                       |
|                      |       | 47    PTI mismatch                              |
|                      |       | 49    Last PDN disconnection not allowed        |
|                      |       | 50    PDN type IPv4 only allowed                |
|                      |       | 51    PDN type IPv6 only allowed                |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                116 / 282

                                          5G Module Series
|       |       | 52    Single address bearers only allowed  |
| ----- | ----- | ------------------------------------------ |
|       |       | 53    ESM information not received         |
|       |       | 54    PDN connection does not exist        |
            55    Multiple PDN connections for a given APN not allowed
|       |       | 56    Collision with network initiated request  |
| ----- | ----- | ----------------------------------------------- |
|       |       | 57    PDN type IPv4v6 only allowed              |
|       |       | 58    PDN type non-IP only allowed              |
|       |       | 59    Unsupported QCI value                     |
|       |       | 60    Bearer handling not supported             |
|       |       | 61    PDN type Ethernet only allowed            |
|       |       | 65    Maximum number of EPS bearers reached     |
            66    Requested APN not supported in current RAT and PLMN
|       |       |     combination                       |
| ----- | ----- | ------------------------------------- |
|       |       | 81    Invalid PTI value               |
|       |       | 95    Semantically incorrect message  |
|       |       | 96    Invalid mandatory information   |
            97    Message type non-existent or not implemented
            98    Message type not compatible with the protocol state
            99    Information element non-existent or not implemented
|       |       | 100   Conditional IE error  |
| ----- | ----- | --------------------------- |
            101   Message not compatible with the protocol state
|       |       | 111   Protocol error, unspecified  |
| ----- | ----- | ---------------------------------- |
            112   APN restriction value incompatible with active EPS bearer context
            113   Multiple accesses to a PDN connection not allowed
| <5gmm_reject_cause>  |       | Integer type. 5GMM reject cause.  |
| -------------------- | ----- | --------------------------------- |
|                      |       | 0    No cause                     |
|                      |       | 3    Illegal UE                   |
|                      |       | 5    PEI not accepted             |
|                      |       | 6    Illegal ME                   |
|                      |       | 7    5GS services not allowed     |
            9    UE identity cannot be derived by the network
|       |       | 10    Implicitly de-registered                   |
| ----- | ----- | ------------------------------------------------ |
|       |       | 11    PLMN not allowed                           |
|       |       | 12    Tracking area not allowed                  |
|       |       | 13    Roaming not allowed in this tracking area  |
|       |       | 15    No suitable cells in tracking area         |
|       |       | 20    MAC failure                                |
|       |       | 21    Synch failure                              |
|       |       | 22    Congestion                                 |
|       |       | 23    UE security capabilities mismatch          |
|       |       | 24    Security mode rejected, unspecified        |
|       |       | 26    Non-5G authentication unacceptable         |
|       |       | 27    N1 mode not allowed                        |
|       |       | 28    Restricted service area                    |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                117 / 282

                                          5G Module Series
|       |       | 31    Redirection to EPC required             |
| ----- | ----- | --------------------------------------------- |
|       |       | 43    LADN not available                      |
|       |       | 62    No network slices available             |
|       |       | 65    Maximum number of PDU sessions reached  |
            67    Insufficient resources for specific slice and DNN
|       |       | 69    Insufficient resources for specific slice  |
| ----- | ----- | ------------------------------------------------ |
|       |       | 71    ngKSI already in use                       |
|       |       | 72    Non-3GPP access to 5GCN not allowed        |
|       |       | 73    Serving network not authorized             |
|       |       | 74    Temporarily not authorized for this SNPN   |
|       |       | 75    Permanently not authorized for this SNPN   |
            76    Not authorized for this CAG or authorized for CAG cells only
|       |       | 77    Wireline access area not allowed  |
| ----- | ----- | --------------------------------------- |
            78    PLMN not allowed to operate at the present UE location
|       |       | 79    UAS services not allowed   |
| ----- | ----- | -------------------------------- |
|       |       | 90    Payload was not forwarded  |
            91    DNN not supported or not subscribed in the slice
            92    Insufficient user-plane resources for the PDU session
|       |       | 95    Semantically incorrect message  |
| ----- | ----- | ------------------------------------- |
|       |       | 96    Invalid mandatory information   |
            97    Message type non-existent or not implemented
            98    Message type not compatible with the protocol state
            99    Information element non-existent or not implemented
|       |       | 100   Conditional IE error  |
| ----- | ----- | --------------------------- |
            101   Message not compatible with the protocol state
|       |       | 111   Protocol error, unspecified  |
| ----- | ----- | ---------------------------------- |
Example
AT+QNETRC=7
OK
AT+QNETRC
+QNETRC: 7

OK
AT+QNETRC?
+QNETRC: "emm_cause",7
+QNETRC: "esm_cause",0
+QNETRC: "5gmm_cause",0

OK

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                118 / 282

5G Module Series
5.23. AT+QNWCFG Configure and Query Network Parameters
This command configures and queries network parameters.
AT+QNWCFG Configure and Query Network Parameters
Test Command Response
AT+QNWCFG=? +QNWCFG: "lte_cell_id",
+QNWCFG: "nr5g_cell_id"
+QNWCFG: "wcdma_cqi"
+QNWCFG: "up/down",(range of supported
<time_interval>s)
+QNWCFG: "dss_enable",(list of supported <enable>s)
OK
Maximum Response Time 300 ms
Characteristics -
5.23.1. AT+QNWCFG="lte_cell_id" Read Cell ID Under LTE
This command reads ECGI, ECI, eNodeB ID under LTE.
AT+QNWCFG="lte_cell_id" Read Cell ID Under LTE
Write Command Response
AT+QNWCFG="lte_cell_id" [+QNWCFG: "lte_cell_id",<ECGI>,<ECI>,<eNodeB_ID>]
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<ECGI> Integer type. E-UTRAN Cell Global Identifier in hexadecimal format (MCC + MNC +
ECI).
<ECI> Integer type. E-UTRAN Cell Identity in hexadecimal format (eNodeB ID + cell ID).
<eNodeB_ID> Integer type. LTE base station ID in hexadecimal format.
Example
AT+QNWCFG="lte_cell_id" //Read cell IDs under LTE.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 119 / 282

5G Module Series
+QNWCFG: "lte_cell_id",64F0000D6B5C0,0D6B5C0,0D6B5
OK
AT+QNWCFG="lte_cell_id" //Read cell ID under non-LTE mode.
OK
5.23.2. AT+QNWCFG="nr5g_cell_id" Read Cell ID Under 5G SA
This command reads the NCGI, NCI, NR5G base station ID under 5G SA.
AT+QNWCFG="nr5g_cell_id" Read Cell ID Under 5G SA
Write Command Response
AT+QNWCFG="nr5g_cell_id" [+QNWCFG: "nr5g_cell_id",<NCGI>,<NCI>,<gNodeB_ID>]
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<NCGI> Integer type. NR Cell Global Identification in hexadecimal format (MCC + MNC +
NCI).
<NCI> Integer type. NR Cell Identification in hexadecimal format (gNodeB ID + cell ID).
<gNodeB_ID> Integer type. 5G base station ID in hexadecimal format.
Example
AT+QNWCFG="nr5g_cell_id" //Read cell IDs under 5G SA.
+QNWCFG: "nr5g_cell_id",64F000170C23000,170C23000,170C23
OK
AT+QNWCFG="nr5g_cell_id" //Read cell ID under non-5G SA.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 120 / 282

5G Module Series
5.23.3. AT+QNWCFG="wcdma_cqi" Read CQI Under WCDMA
This command reads CQI (Channel Quality Indicator) under WCDMA.
AT+QNWCFG="wcdma_cqi" Read CQI Under WCDMA
Write Command Response
AT+QNWCFG="wcdma_cqi" +QNWCFG: "wcdma_cqi",<CQI_value>
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<CQI_value> Integer type. CQI value. Range: 0–30 and 255. If 255 is returned, it means that CQI in
WCDMA is invalid.
NOTE
The CQI value can be obtained after the HSDPA channel is created, and the HSDPA channel can be
established by testing the data traffic.
Example
AT+QNWCFG="wcdma_cqi"
+QNWCFG: "wcdma_cqi",27
OK
5.23.4. AT+QNWCFG="up/down" Get Average Uplink and Downlink Rates in Delta
Time
This command gets average uplink rate and downlink rate in delta time.
AT+QNWCFG="up/down" Get Average Uplink Rate and Downlink Rate in Delta Time
Write Command Response
AT+QNWCFG="up/down"[,<ti If the optional parameter is omitted, query the current setting:
me_interval>] +QNWCFG: "up/down",<uplink>,<downlink>,<time_interval>
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 121 / 282

5G Module Series
If the optional parameter is specified, set interval time for
automatically calculating the average rate:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is not saved.
Parameter
<uplink> Integer type. Average uplink rate in delta time. Unit: bits/second.
<downlink> Integer type. Average downlink rate in delta time. Unit: bits/second.
<time_interval> Integer type. Time required to calculate the average rate automatically. Range:1–
60. Default value: 2. Unit: second.
NOTE
Executing AT+QNWCFG=”up/down” writes data to NVM. Please proceed with caution.
Example
AT+QNWCFG="up/down" //Query the current setting.
+QNWCFG: "up/down",2056,384,2
OK
AT+QNWCFG="up/down",5 //Set the interval time for automatically calculating the average rate.
OK
5.23.5. AT+QNWCFG="dss_enable" Enable or Disable DSS Function
This command enables or disables DSS Function.
AT+QNWCFG="dss_enable" Enable/Disable DSS Function
Write Command Response
AT+QNWCFG="dss_enable"[, If the optional parameter is omitted, query the current setting:
<enable>] +QNWCFG: "dss_enable",<enable>
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 122 / 282

5G Module Series
If the optional parameter is specified, enable or disable DSS:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<enable> Integer type. Enable or disable DSS function.
0 Disable
1 Enable
Example
AT+QNWCFG="dss_enable",1 //Enable DSS function.
OK
AT+QNWCFG="dss_enable" //Query whether DSS is enabled.
+QNWCFG: "dss_enable",1
OK
5.24. AT+QNWPREFCFG Configure Network Searching Preferences
This command configures the network searching preferences.
AT+QNWPREFCFG Configure Network Searching Preferences
Test Command Response
AT+QNWPREFCFG=? +QNWPREFCFG: "gw_band",(list of supported <gw_band>s)
+QNWPREFCFG: "lte_band",(list of supported <LTE_band>s)
+QNWPREFCFG: "nsa_nr5g_band",(list of supported <NSA_N
R5G_band>s)
+QNWPREFCFG: "nr5g_band",(list of supported <SA_NR5G_b
and>s)
+QNWPREFCFG: "mode_pref",(list of supported <mode_pref>
s)
+QNWPREFCFG: "srv_domain",(list of supported <srv_domai
n>s)
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 123 / 282

5G Module Series
+QNWPREFCFG: "voice_domain",(list of supported <voice_do
main>s)
+QNWPREFCFG: "roam_pref",(list of supported <roam_pref>s)
+QNWPREFCFG: "ue_usage_setting",(list of supported <settin
g>s)
+QNWPREFCFG: "policy_band"
+QNWPREFCFG: "ue_capability_band"
+QNWPREFCFG: "rat_acq_order",(list of supported <rat_orde
r>s)
+QNWPREFCFG: "nr5g_disable_mode",(list of supported <dis
able_mode>s)
OK
Maximum Response Time 300 ms
Characteristics -
5.24.1. AT+QNWPREFCFG="gw_band" Set WCDMA Band
This command specifies the preferred WCDMA band to be searched by UE.
AT+QNWPREFCFG="gw_band" Set WCDMA Band
Write Command Response
AT+QNWPREFCFG="gw_band" If the optional parameter is omitted, query the current setting:
[,<gw_band>] +QNWPREFCFG: "gw_band",<gw_band>
OK
If the optional parameter is specified, set the preferred WCDMA
bands to be searched:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<gw_band> String type. WCDMA bands to be configured. Format:
<WCDMA_band1>:<WCDMA_band2>:…:<WCDMA_bandn>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 124 / 282

5G Module Series
<WCDMA_band> Integer type. WCDMA band.
1 WCDMA 2100 band
2 WCDMA 1900 band
3 WCDMA 1800 band
4 WCDMA 1700 band
5 WCDMA 850 band
6 WCDMA 800 band
8 WCDMA 900 band
19 WCDMA Japan 850 band
NOTE
1. See the specific module specification for the bands that can be supported.
2. When the module locks to WCDMA, an error is reported if <gw_band> is set to null.
3. Executing AT+QNWPREFCFG="gw_band",<gw_band> writes data to NVM. Please proceed with
caution.
Example
AT+QNWPREFCFG="gw_band" //Query the configured WCDMA bands of the UE.
+QNWPREFCFG: "gw_band",1:2:3:4:5:6:7:8:9:19
OK
AT+QNWPREFCFG="gw_band",1:2 //Set WCDMA B1 and B2.
OK
5.24.2. AT+QNWPREFCFG="lte_band" Set LTE Band
This command specifies the preferred LTE band to be searched by UE.
AT+QNWPREFCFG="lte_band" Set LTE Band
Write Command Response
AT+QNWPREFCFG="lte_band" If the optional parameter is omitted, query the current setting:
[,<LTE_band>] +QNWPREFCFG: "lte_band",<LTE_band>
OK
If the optional parameter is specified, set the preferred LTE bands
to be searched:
OK
If there is any error:
ERROR
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 125 / 282

5G Module Series
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<LTE_band> String type. LTE bands to be configured. Format: <band1>:<band2>:…:<bandn>.
<band> Integer type. LTE bands supported by the module.
1–5 B1–B5
7 B7
8 B8
12–14 B12–B14
17–20 B17–B20
25 B25
26 B26
28–30 B28–B30
32 B32
34 B34
38–43 B38–B43
48 B48
66 B66
71 B71
NOTE
1. See the specific module specification for the bands that are supported.
2. When the module locks to LTE, an error is reported if <LTE_band> is set to null.
3. Executing AT+QNWPREFCFG="lte_band",<LTE_band> writes data to NVM. Please proceed with
caution.
Example
AT+QNWPREFCFG="lte_band" //Query the configured LTE bands of the UE.
+QNWPREFCFG: "lte_band",1:2:3:4:5:7:8:12:13:14:17:18:19:20:25:26:28:29:30:32:34:38:39:40:41:
42:66:71
OK
AT+QNWPREFCFG="lte_band",1:2 //Set LTE B1 and LTE B2.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 126 / 282

5G Module Series
5.24.3. AT+QNWPREFCFG="nsa_nr5g_band" Set 5G NSA Band
This command specifies the preferred 5G NSA bands to be searched by UE.
AT+QNWPREFCFG="nsa_nr5g_band" Set 5G NSA Band
Write Command Response
AT+QNWPREFCFG="nsa_nr5g If the optional parameter is omitted, query the current setting:
_band"[,<NSA_NR5G_band>] +QNWPREFCFG: "nsa_nr5g_band",<NSA_NR5G_band>
OK
If the optional parameter is specified, set the preferred 5G NSA
bands to be searched:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<NSA_NR5G_band> String type. 5G NSA bands to be configured. Format:
<NSA_band1>:<NSA_band2>:…:<NSA_bandn>
<NSA_band> Integer type. 5G NSA band. The configurable 5G NSA bands supported by the
module.
1–3 n1–n3
5 n5
7 n7
8 n8
12 n12
20 n20
25 n25
28 n28
38 n38
40 n40
41 n41
48 n48
66 n66
71 n71
77–79 n77–n79
257 n257
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 127 / 282

5G Module Series
258 n258
260 n260
261 n261
NOTE
1. See the specific module specification for the bands that are supported.
2. When the module locks to 5G NSA, an error is reported if <NSA_NR5G_band> is set to null.
3. Executing AT+QNWPREFCFG="nsa_nr5g_band",<NSA_NR5G_band> writes data to NVM.
Please proceed with caution.
Example
AT+QNWPREFCFG= "nsa_nr5g_band" //Query the currently configured 5G NSA bands of UE.
+QNWPREFCFG: "nsa_nr5g_band",1:3:7:20:28:40:41:71:77:78:79
OK
AT+QNWPREFCFG= "nsa_nr5g_band",1:2 //Set 5G NSA n1 and 5G NSA n2.
OK
5.24.4. AT+QNWPREFCFG="nr5g_band" Set 5G SA Band
This command specifies the preferred 5G SA band to be searched by UE.
AT+QNWPREFCFG="nr5g_band" Set 5G SA Band
Write Command Response
AT+QNWPREFCFG="nr5g_ban If the optional parameter is omitted, query the current setting:
d"[,<SA_NR5G_band>] +QNWPREFCFG: "nr5g_band",<SA_NR5G_band>
OK
If the optional parameter is specified, set the preferred NR5G SA
bands to be searched:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 128 / 282

5G Module Series
Parameter
<SA_NR5G_band> String type. 5G NSA bands to be configured. Format:
<SA_band1>:<SA_band2>:…:<SA_bandn>.
<SA_band> Integer type. SA 5G band. The configurable SA 5G bands supported by the
applicable modules.
1–3 n1–n3
7 n7
8 n8
12 n12
20 n20
25 n25
28 n28
38 n38
40 n40
41 n41
48 n48
66 n66
71 n71
77–79 n77–n79
NOTE
1. See the specific module specification for the bands that are supported by the module.
2. When the module locks to 5G SA, an error is reported if <SA_NR5G_band> is set to null.
3. Executing AT+QNWPREFCFG="nr5g_band",<SA_NR5G_band> writes data to NVM. Please
proceed with caution.
Example
AT+QNWPREFCFG= "nr5g_band" //Query the currently configured 5G SA bands of the UE.
+QNWPREFCFG: "nr5g_band",1:3:7:20:28:40:41:71:77:78:79
OK
AT+QNWPREFCFG= "nr5g_band",1:2 //Set 5G SA n1 and 5G SA n2.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 129 / 282

5G Module Series
5.24.5. AT+QNWPREFCFG="mode_pref" Set Network Search Mode
This command specifies the network search mode.
AT+QNWPREFCFG="mode_pref" Set Network Search Mode
Write Command Response
AT+QNWPREFCFG="mode_pre If the optional parameter is omitted, query the current setting:
f"[,<mode_pref>] +QNWPREFCFG: "mode_pref",<mode_pref>
OK
If the optional parameter is specified, set the network search mode:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<mode_pref> String type without double quotes. RATs to be configured.
Format: <mode_pref1>:<mode_pref2>:…:<mode_prefn>. RATs supported:
AUTO WCDMA & LTE & 5G
WCDMA WCDMA only
LTE LTE only
NR5G 5G only
NOTE
Executing AT+QNWPREFCFG="mode_pref",<mode_pref> writes data to NVM. Please proceed with
caution.
Example
AT+QNWPREFCFG="mode_pref" //Query the current setting.
+QNWPREFCFG: "mode_pref",AUTO
OK
AT+QNWPREFCFG="mode_pref",LTE //Set RAT to LTE only.
OK
AT+QNWPREFCFG="mode_pref",LTE:NR5G //Set RAT to LTE & 5G.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 130 / 282

5G Module Series
OK
5.24.6. AT+QNWPREFCFG="srv_domain" Set Service Domain
This command specifies the registered service domain.
AT+QNWPREFCFG="srv_domain" Set Service Domain
Write Command Response
AT+QNWPREFCFG="srv_doma If the optional parameter is omitted, query the current setting:
in"[,<srv_domain>] +QNWPREFCFG: "srv_domain",<srv_domain>
OK
If the optional parameter is specified, set the service domain of UE:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<srv_domain> Integer type. UE service domain.
0 CS only
1 PS only
2 CS & PS
NOTE
Executing AT+QNWPREFCFG="srv_domain",<srv_domain> writes data to NVM. Please proceed
with caution.
Example
AT+QNWPREFCFG="srv_domain" //Query the current setting.
+QNWPREFCFG: "srv_domain",2
OK
AT+QNWPREFCFG="srv_domain",1 //Set PS only.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 131 / 282

5G Module Series
5.24.7. AT+QNWPREFCFG="voice_domain" Set Voice Domain
This command specifies the UE voice domain.
AT+QNWPREFCFG="voice_domain Set Voice Domain
Write Command Response
AT+QNWPREFCFG="voice_do If the optional parameter is omitted, query the current setting:
main"[,<voice_domain>] +QNWPREFCFG: "voice_domain",<voice_domain>
OK
If the optional parameter is specified, set UE voice domain:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<voice_domain> Integer type. UE voice domain.
0 CS voice only
1 IMS PS voice only
2 CS voice preferred with IMS PS voice as secondary
3 IMS PS voice preferred with CS voice as secondary
NOTE
Executing AT+QNWPREFCFG="voice_domain",<voice_domain> writes data to NVM. Please
proceed with caution.
Example
AT+QNWPREFCFG="voice_domain" //Query the current configuration.
+QNWPREFCFG: "voice_domain",2
OK
AT+QNWPREFCFG="voice_domain",3 //Set IMS voice preferred.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 132 / 282

5G Module Series
5.24.8. AT+QNWPREFCFG="roam_pref" Set Roaming Preference
This command specifies the roaming preference of UE.
AT+QNWPREFCFG="roam_pref" Set Roaming Preference
Write Command Response
AT+QNWPREFCFG="roam_pr If the optional parameter is omitted, query the current setting:
ef"[,<roam_pref>] +QNWPREFCFG: "roam_pref",<roam_pref>
OK
If the optional parameter is specified, set UE roaming preference:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<roam_pref> Integer type. UE roaming preference.
1 Roam only on home network
3 Roam on affiliate network
255 Roam on any network
NOTE
Executing AT+QNWPREFCFG="roam_pref",<roam_pref> writes data to NVM. Please proceed with
caution.
Example
AT+QNWPREFCFG="roam_pref" //Query the current setting.
+QNWPREFCFG: "roam_pref",255
OK
AT+QNWPREFCFG= "roam_pref",1 //Roam only on home network.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 133 / 282

5G Module Series
5.24.9. AT+QNWPREFCFG="ue_usage_setting" Set UE Usage Setting
This command specifies the usage setting of UE.
AT+QNWPREFCFG="ue_usage_setting" Set UE Usage Setting
Write Command Response
AT+QNWPREFCFG="ue_usage If the optional parameter is omitted, query the current setting:
_setting"[,<setting>] +QNWPREFCFG: "ue_usage_setting",<setting>
OK
If the optional parameter is specified, set UE usage setting:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<setting> Integer type. Usage setting of UE .
0 Voice centric
1 Data centric
NOTE
Executing AT+QNWPREFCFG="ue_usage_setting",<setting> writes data to NVM. Please proceed
with caution.
Example
AT+QNWPREFCFG="ue_usage_setting" //Query the current setting.
+QNWPREFCFG: "ue_usage_setting",1
OK
AT+QNWPREFCFG="ue_usage_setting",0 //Set voice centric.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 134 / 282

5G Module Series
5.24.10. AT+QNWPREFCFG="policy_band" Read Carrier Policy Band
This command reads the band configured in the carrier policy.
AT+QNWPREFCFG="policy_band" Read Carrier Policy Band
Write Command Response
AT+QNWPREFCFG="policy_band" +QNWPREFCFG: "gw_band",<gw_band>
+QNWPREFCFG: "lte_band",<LTE_band>
+QNWPREFCFG: "nsa_nr5g_band",<NSA_NR5G_ban
d>
+QNWPREFCFG: "nr5g_band",<SA_NR5G_band>
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<gw_band> String type. WCDMA bands to be configured. Format:
<WCDMA_band1>:<WCDMA_band2>:…:<WCDMA_bandn>
<WCDMA_band> Integer type. WCDMA band. See <WCDMA_band> in Chapter 5.24.1.
<LTE_band> String type. LTE bands to be configured. Format:
<band1>:<band2>:…:<bandn>.
<band> Integer type. LTE band. See <band> in Chapter 5.24.2.
<NSA_NR5G_band> String type. 5G NSA bands to be configured. Format:
<NSA_band1>:<NSA_band1>:…:<NSA_bandn>
<NSA_band> Integer type. 5G NSA band. See <NSA_band> in Chapter 5.24.3.
<SA_NR5G_band> String type. 5G SA bands to be configured. Format:
<SA_band1>:<SA_band2>:…:<SA_bandn>
<SA_band> Integer type. 5G SA band. See <SA_band> in Chapter 5.24.4.
NOTE
See the specific module specification for the bands supported.
Example
AT+QNWPREFCFG="policy_band"
+QNWPREFCFG: "gw_band",1:8
+QNWPREFCFG: "lte_band",1:3:8
+QNWPREFCFG: "nsa_nr5g_band",78
+QNWPREFCFG: "nr5g_band",78
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 135 / 282

5G Module Series
OK
5.24.11. AT+QNWPREFCFG="ue_capability_band" Query UE Band Capability
This command queries the band configured in the UE capability.
AT+QNWPREFCFG="ue_capability_band" Query UE Band Capability
Write Command Response
AT+QNWPREFCFG="ue_capability_ +QNWPREFCFG: "gw_band",<gw_band>
band" +QNWPREFCFG: "lte_band",<LTE_band>
+QNWPREFCFG: "nsa_nr5g_band",<NSA_NR5G_band>
+QNWPREFCFG: "nr5g_band",<SA_NR5G_band>
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<gw_band> String type. Use the colon as a separator to list the WCDMA bands to be
configured. Parameter format::
<WCDMA_band1>:<WCDMA_band2>:…:<WCDMA_bandn>
<WCDMA_band> Integer type. WCDMA band. See <WCDMA_band> in Chapter 5.24.1.
<LTE_band> String type. Use the colon as a separator to list the LTE bands to be
configured. Parameter format: <band1>:<band2>:…:<bandn>.
<band> Integer type. LTE band. See <band> in Chapter 5.24.2.
<NSA_NR5G_band> String type. Use the colon as a separator to list the NR5G NSA bands to be
configured.
Parameter format: <NSA_band1>:<NSA_band1>:…:<NSA_bandn>
<NSA_band> Integer type. 5G NSA band. See <NSA_band> in Chapter 5.24.3.
<SA_NR5G_band> String type. Use the colon as a separator to list the NR5G SA bands to be
configured. Parameter format: <SA_band1>:<SA_band2>:…:<SA_bandn>
<SA_band> Integer type. 5G SA band. See <SA_band> in Chapter 5.24.4.
NOTE
Please see the module specification for the bands supported by the specific module.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 136 / 282

5G Module Series
Example
AT+QNWPREFCFG="ue_capability_band"
+QNWPREFCFG: "gw_band",1:8
+QNWPREFCFG: "lte_band",1:3:8
+QNWPREFCFG: "nsa_nr5g_band",78
+QNWPREFCFG: "nr5g_band",78
OK
5.24.12. AT+QNWPREFCFG="rat_acq_order" Set RAT Priority
This command sets the RAT acquisition order.
AT+QNWPREFCFG="rat_acq_order" Set RAT Priority
Write Command Response
AT+QNWPREFCFG="rat_acq_o If the optional parameter is omitted, query the current setting:
rder"[,<rat_order>] +QNWPREFCFG: "rat_acq_order",<rat_order>
OK
If the optional parameter is specified, set the RAT acquisition order:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<rat_order> String type without double quotes. RAT priority.
Format: <rat_order1>:<rat_order2>:…:< rat_ordern>. RATs supported:
WCDMA
LTE
NR5G
NOTE
Executing AT+QNWPREFCFG="rat_acq_order",<rat_order> writes data to NVM. Please proceed
with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 137 / 282

5G Module Series
Example
AT+QNWPREFCFG= "rat_acq_order" //Query the current RAT order.
+QNWPREFCFG: "rat_acq_order",NR5G:LTE:WCDMA
OK
AT+QNWPREFCFG= "rat_acq_order",LTE:NR5G:WCDMA //Set RAT order priority.
OK
AT+CFUN=1,1 //Reset the module.
OK
AT+QNWPREFCFG= "rat_acq_order" //Query the current RAT order.
+QNWPREFCFG: "rat_acq_order", LTE:NR5G:WCDMA
OK
5.24.13. AT+QNWPREFCFG="nr5g_disable_mode" Disable 5G
This command disables 5G.
AT+QNWPREFCFG="nr5g_disable_mode" Disable 5G
Write Command Response
AT+QNWPREFCFG="nr5g_disa If the optional parameter is omitted, query the current setting:
ble_mode"[,<disable_mode>] +QNWPREFCFG: "nr5g_disable_mode",<disable_mode>
OK
If the optional parameter is specified, disable NR5G:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<disable_mode> Integer type. Disable 5G SA/NSA.
0 Neither is disabled
1 Disable 5G SA
2 Disable 5G NSA
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 138 / 282

5G Module Series
NOTE
Executing AT+QNWPREFCFG="nr5g_disable_mode",<disable_mode> writes data to NVM. Please
proceed with caution.
Example
AT+QNWPREFCFG="nr5g_disable_mode" //Query the current configuration.
+QNWPREFCFG: "nr5g_disable_mode",0
OK
AT+QNWPREFCFG="nr5g_disable_mode",1 //Disable 5G SA.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 139 / 282

5G Module Series
6
Call Related Commands
6.1. ATA Answer an Incoming Call
This command connects the MT to an incoming voice or data call indicated by a RING URC.
ATA Answer an Incoming Call
Execution Command Response
ATA MT sends off-hook to the remote station.
In case of a data call, if successfully connected:
CONNECT<text>
MT switches to data mode.
<text> is output only when <value> of ATX is greater than 0.
When MT returns to command mode after call release:
OK
In case of a voice call, if successfully connected:
OK
If no call connection:
NO CARRIER
Maximum Response Time 90 s, determined by the network.
Characteristics -
Reference ITU-T Recommendation V.25 ter
NOTE
1. Any additional commands on the same command line are ignored.
2. This command may be aborted when the module receives a character during command execution.
However, the command will not be aborted during some connection establishment processes such
as handshaking.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 140 / 282

5G Module Series
Example
RING##0 //Incoming call.
AT+CLCC
+CLCC: 1,0,0,1,0,"",129 //PS call in LTE mode.
+CLCC: 2,1,4,0,0,"02154450290",129 //Incoming call.
OK
ATA //Answer a voice call with ATA.
OK
6.2. ATD Originate a Call
This command sets up an outgoing voice or a data call. Supplementary services can also be controlled
with this command.
ATD Originate a Call
Execution Command Response
ATD<n>[<mgsm>][;] If no dial tone and ATX2 or ATX4 is set:
NO DIALTONE
If busy and ATX3 or ATX4 is set:
BUSY
If a call connection cannot be established:
NO CARRIER
If a call connection is established successful and a non-voice call
is to be set:
CONNECT<text>
MT switches to data mode.
<text> is output only when <value> of ATX is greater than 0.
When MT returns to command mode after call release:
OK
If a call connection is established successful and a voice call is
set up:
OK
If there is any error:
ERROR
Maximum Response Time 5 s, determined by the network.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 141 / 282

5G Module Series
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> String of dialing digits and optional V.25ter modifiers.
Dialing digits: 0-9, *, #, +, A, B, C
Following V.25ter, optional modifiers ,(comma), T, P, !, W, @ are ignored
<mgsm> String of GSM modifiers:
I Activates CLIR (presentation of own number to called party disabled)
i Deactivates CLIR (presentation of own number to called party enabled)
G Activates CUG invocation for this call only
g Deactivates CUG invocation for this call only
<;> It is required when setting up a voice call, and MT will return to command state after
call release.
NOTE
1. This command may be aborted during execution if the module receives an ATH command or a
character. However, the command will not be aborted during certain connection establishments such
as handshaking.
2. "I" and "i" of <mgsm> are only valid when <n> doesn’t contain "*" or "#".
3. See ATX for setting result codes and call monitoring parameters
.
4. For voice calls, if dialing with ATD, there are two possible response modes:
MT returns OK immediately, either after dialing is completed or after the call is established. The
mode is controlled by AT+COLP. By default, AT+COLP=0 is set, causing the MT to return OK
immediately after the dialing is completed. Otherwise, MT returns BUSY, NO DIAL TONE, or NO
CARRIER.
5. Using ATD during an active voice call:
⚫ When a user originates a second voice call while an active voice call is in progress, the first call
will be automatically put on hold.
⚫ The current states of all calls can be easily checked at any time with AT+CLCC.
Example
ATD10086; //Dial a number.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 142 / 282

5G Module Series
6.3. ATH Disconnect Existing Call
This command disconnects data or voice call. AT+CHUP is also used for disconnecting voice call.
ATH Disconnect Existing Call
Execution Command Response
ATH[<n>] OK
Maximum Response Time 90 s, determined by the network.
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type.
0 Disconnect existing call from command line
6.4. AT+CVHU Control Voice Call Hang Up
This command controls whether ATH can be used to disconnect existing voice call.
AT+CVHU Control Voice Call Hang Up
Test Command Response
AT+CVHU=? +CVHU: (list of supported <mode>s)
OK
Read Command Response
AT+CVHU? +CVHU: <mode>
OK
Write Command Response
AT+CVHU=<mode> OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 143 / 282

5G Module Series
Parameter
<mode> Integer type.
0 ATH can be used to disconnect existing voice call
1 ATH is ignored with only the response OK returned
6.5. AT+CHUP Hang Up Voice Call
This command cancels any voice call in Active, Waiting, and Held state. For data call hang up, use ATH.
AT+CHUP Hang Up Voice Calls
Test Command Response
AT+CHUP=? OK
Execution Command Response
AT+CHUP OK
Or
ERROR
Maximum Response Time 90 s, determined by the network.
Characteristics -
Reference 3GPP TS 27.007
Example
RING //Incoming call.
AT+CHUP //Hang up the call.
OK
6.6. ATS0 Set Ring Count Before Automatic Answering
This command controls automatic answering mode for incoming call.
ATS0 Set Ring Count Before Automatic Answering
Read Command Response
ATS0? <n>
OK
Write Command Response
ATS0=<n> OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 144 / 282

                                          5G Module Series
Or
ERROR
| Maximum Response Time  |     |     | 300 ms                         |
| ---------------------- | --- | --- | ------------------------------ |
| Characteristics        |     |     | -                              |
| Reference              |     |     | ITU-T Recommendation V.25 ter  |
Parameter
| <n>     | Integer type.                      |     |     |
| ------- | ---------------------------------- | --- | --- |
|         | 0    Disable automatic answering   |     |     |
      1–255  Enable automatic answering on specified ring count

NOTE
If <n> is set to a high value (based on operator need, usually 90 s), the calling party may hang up before
| the call is answered automatically. |     |     |     |
| ----------------------------------- | --- | --- | --- |
Example
ATS0=3                 //Set three rings before automatically answering a call.
OK

| RING    |       |       |   //Call incoming.  |
| ------- | ----- | ----- | ------------------- |
##0

RING
##0

RING                  //Call is automatically answered after three rings.
##0

6.7.  ATS6  Set Pause Before Blind Dialing

This command is implemented for compatibility reason only, and has no effect.
ATS6  Set Pause Before Blind Dialing
| Read Command  |     |     | Response  |
| ------------- | --- | --- | --------- |
| ATS6?         |     |     | <n>       |

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                145 / 282

5G Module Series
OK
Write Command Response
ATS6=<n> OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type. Number of seconds to wait before blind dialing. Range: 0–10. Default: 2.
6.8. ATS7 Set Waiting Time for Connection Completion
This command specifies the duration (unit: second) to wait for the connection completion in case of
answering or originating a call. If no connection is established during the time, MT will be disconnected.
ATS7 Set Waiting Time for Connection Completion
Read Command Response
ATS7? <n>
OK
Write Command Response
ATS7=<n> OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type.
0 Disabled
1–255 Waiting time in second(s) for connection completion
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 146 / 282

5G Module Series
6.9. ATS8 Set Waiting Time for Comma Dial Modifier
This command is implemented for compatibility reason only, and has no effect.
ATS8 Set Waiting Time for Comma Dial Modifier
Read Command Response
ATS8? <n>
OK
Write Command Response
ATS8=<n> OK
Maximum Response Time 300 ms
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type.
0 No pause when comma encountered in dial string
1–2–255 Waiting time in second(s) for comma dial modifier
6.10. ATS10 Set Disconnection Delay After Indicating Data Carrier
Absence
This command determines the duration (unit: tenth(s) of a second) during which UE remains connected
when no data carrier is present. If the data carrier is detected again before disconnection, MT remains
connected.
ATS10 Set Disconnection Delay After Indicating Data Carrier Absence
Read Command Response
ATS10? <n>
OK
Write Command Response
ATS10=<n> OK
Maximum Response Time 300 ms
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 147 / 282

5G Module Series
Characteristics -
Reference ITU-T Recommendation V.25 ter
Parameter
<n> Integer type. Time to wait before disconnecting after UE has indicated the absence of received
line signal. Range: 1–254. Default: 15. Unit: tenth(s) of a second.
6.11. AT+CSTA Select Address Type
This command selects the type of number for further dialing command ATD according to 3GPP
specifications. The Test Command returns supported values as a compound value.
AT+CSTA Select Address Type
Test Command Response
AT+CSTA=? +CSTA: (list of supported <type>s)
OK
Read Command Response
AT+CSTA? +CSTA: <type>
OK
Write Command Response
AT+CSTA=[<type>] OK
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<type> Integer type. Current address type setting.
129 Unknown type
145 International type (contains the character "+")
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 148 / 282

5G Module Series
6.12. AT+CLCC List Current Call of MT
This command returns the list of all current call(s). If the command is executed successfully, but there is
no active call, no information will be provided in the response, and only OK will be sent to TE.
AT+CLCC List Current Call of MT
Test Command Response
AT+CLCC=? OK
Execution Command Response
AT+CLCC [+CLCC:
<id>,<dir>,<stat>,<mode>,<mpty>[,<number>,<type>[,<al
pha>]]]
[...]
OK
If there is any error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Parameter
<id> Integer type. Call identification number as described in 3GPP TS 22.030, which can be
used in AT+CHLD operation.
<dir> Integer type.
0 Mobile originated (MO) call
1 Mobile terminated (MT) call
<stat> Integer type. Call state.
0 Active
1 Held
2 Dialing (MO call)
3 Alerting (MO call)
4 Incoming (MT call)
5 Waiting (MT call)
<mode> Integer type. Bearer/teleservice.
0 Voice
1 Data
2 Fax
<mpty> Integer type.
0 Call is not a part of a multiparty (conference) call
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 149 / 282

                                          5G Module Series
|       | 1    Call is a part of a multiparty (conference) call   |     |     |
| ----- | ------------------------------------------------------- | --- | --- |
<number>   String type. Phone number. Format specified by <type>.
<type>  Integer type. Octet address type (See 3GPP TS 24.008 subclause 10.5.4.7 for details).
|       | 129   Unknown type                                     |     |     |
| ----- | ------------------------------------------------------ | --- | --- |
|       | 145   International type (contains the character "+")  |     |     |
|       | 161   National type                                    |     |     |
<alpha>   String type. Alphanumeric representation for <number> corresponding to the entry found
in phonebook.
| <err>    | Error code. For more details, see Chapter 13.5.  |     |     |
| -------- | ------------------------------------------------ | --- | --- |
Example
| ATD10086;  |       |       |   //Dial a number.  |
| ---------- | ----- | ----- | ------------------- |
OK
AT+CLCC
| +CLCC: 1,0,0,1,0,"",129  |     |       |   //PS call in LTE mode.  |
| ------------------------ | --- | ----- | ------------------------- |
+CLCC: 2,0,0,0,0,"10086",129       //Established a call, and the call has been answered.

OK

6.13. AT+CR  Service Reporting Control

This command controls whether MT transmits an intermediate result code +CR: <serv> to TE or not
during call setup.

If  it  is  enabled,  the  intermediate  result  code  is  transmitted  during  the  connect  negotiation  phase,
indicating the selected speed and quality of service, before any error control or data compression is
reported and before any final result code (e.g. CONNECT) is transmitted.
AT+CR  Service Reporting Control
| Test Command  |     |     | Response                          |
| ------------- | --- | --- | --------------------------------- |
| AT+CR=?       |     |     | +CR: (list of supported <mode>s)  |

OK
| Read Command  |     |     | Response     |
| ------------- | --- | --- | ------------ |
| AT+CR?        |     |     | +CR: <mode>  |

OK
| Write Command  |     |     | Response  |
| -------------- | --- | --- | --------- |
AT+CR=[<mode>]  MT  controls  whether  the  intermediate  result  code  +CR:
<serv> is returned by TA to TE or not during call setup.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                150 / 282

                                          5G Module Series
Maximum Response Time  300 ms
Characteristics  -
Reference  3GPP TS 27.007
Parameter
| <mode>    | Integer type.  |                                 |
| --------- | -------------- | ------------------------------- |
|           | 0              |   Disable                       |
|           |  1             |   Enable                        |
| <serv>    | String type.   |                                 |
|           | ASYNC          |   Asynchronous transparent      |
|           | SYNC           |   Synchronous transparent       |
|           | REL ASYNC      | Asynchronous non-transparent    |
|           | REL SYNC       |   Synchronous non-transparent   |

6.14. AT+CRC  Set Extended Format of Incoming Call Indication

This command controls whether to use the extended format of incoming call indication or not. When it is
enabled, an incoming call is indicated to TE with URC +CRING: <type> instead of the usual RING
notification.
AT+CRC  Set Extended Format of Incoming Call Indication
Test Command  Response
AT+CRC=?  +CRC: (list of supported <mode>s)

OK
Read Command  Response
AT+CRC?  +CRC: <mode>

OK
Write Command  Response
AT+CRC=[<mode>]  OK
Maximum Response Time  300 ms
This command takes effect immediately.
Characteristics
The configuration is not saved.
Reference  3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                151 / 282

                                          5G Module Series
Parameter
| <mode>    | Integer type.  |                                |     |
| --------- | -------------- | ------------------------------ | --- |
|           | 0              |   Disable extended format      |     |
|           |  1             |   Enable extended format       |     |
| <type>    | String type.   |                                |     |
|           | ASYNC          |   Asynchronous transparent     |     |
|           | SYNC           |   Synchronous transparent      |     |
|           | REL ASYNC      | Asynchronous non-transparent   |     |
|           | REL SYNC       |   Synchronous non-transparent  |     |
|           | FAX            |   Facsimile                    |     |
|           | VOICE          |   Voice                        |     |
Example
| AT+CRC=1  |       |       |   //Enable extended format.  |
| --------- | ----- | ----- | ---------------------------- |
OK

+CRING: VOICE             //Indicate a voice type incoming call to TE.
ATH
OK
| AT+CRC=0  |       |       |   //Disable extended format.  |
| --------- | ----- | ----- | ----------------------------- |
OK

| RING    |       |       |   //Indicate incoming call to TE.  |
| ------- | ----- | ----- | ---------------------------------- |
ATH
OK

6.15. AT+CRLP  Select Radio Link Protocol Parameter

This command selects radio link protocol (RLP) parameters used when non-transparent data call is
originated.

This Test Command returns supported values. RLP versions 0 and 1 share the same parameter set. MT
returns only one line for this set, omitting <ver>.

This Read Command returns current setting for RLP version. RLP versions 0 and 1 share the same
parameter set. TA returns only one line for this set, excluding <ver>.
AT+CRLP  Select Radio Link Protocol Parameter
| Test Command  |     |     | Response  |
| ------------- | --- | --- | --------- |
AT+CRLP=?  +CRLP: (list of supported <iws>s),(list of supported <mws>s),
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                152 / 282

5G Module Series
(list of supported <T1>s),(list of supported <N2>s),<ver>
+CRLP: (list of supported <iws>s),(list of supported <mws>s),
(list of supported <T1>s),(list of supported <N2>s),<ver>
+CRLP: (list of supported <iws>s),(list of supported <mws>s),
(list of supported <T1>s),(list of supported <N2>s),<ver>
OK
Read Command Response
AT+CRLP? +CRLP: <iws>,<mws>,<T1>,<N2>,<ver>
[...]
OK
Write Command Response
AT+CRLP=[<iws>[,<mws>[,<T1>[,< OK
N2>[,<ver>]]]]] Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<iws> Integer type. Interworking window size (IWF to MS window size).
<ver>=0, 1 Range: 0–61. Default: 61.
<ver>=2 Range: 0–488. Default: 240.
<mws> Integer type. Mobile window size (MS to IWF window size).
<ver>=0, 1 Range: 0–61. Default: 61.
<ver>=2 Range: 0–488. Default: 240
<T1> Integer type. Acknowledgment timer T1 in a unit of 10 ms.
<ver>=0, 1 Range: 38–255. Default: 48.
<ver>=2 Range: 42–255. Default: 52.
<N2> Integer type. Retransmission attempt(s). Range: 1–55. Default: 6.
<ver> Integer type. RLP version number. Range: 0–2.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 153 / 282

5G Module Series
6.16. AT+QECCNUM Configure Emergency Call Number
This command queries, adds and deletes emergency call number (ECC number).
AT+QECCNUM Configure Emergency Call Number
Test Command Response
AT+QECCNUM=? +QECCNUM: (list of supported <mode>s)
OK
Write Command Response
AT+QECCNUM=<mode>[,<type>[,<e If <mode>=0, <type> is specified and <eccnum> is omitted,
ccnum1>[,<eccnum2>[,…[,<eccnum query the current ECC number type:
n>]]]]] +QECCNUM: <type>,<eccnum1>,<eccnum2>[,…]
OK
If <mode>=1, <type>=0 or 1, and at least one <eccnum> is
specified, add ECC number with (U)SIM card or without
(U)SIM card:
OK
If <mode>=2, <type>=0 or 1, and at least one <eccnum> is
specified, delete ECC number with (U)SIM card or without
(U)SIM card:
OK
If <mode>=3, and both <type> and <eccnum> are omitted,
reset ECC number, and the reset will take effect after
rebooting:
OK
If there is any error:
ERROR
Write Command Response
AT+QECCNUM=<mode>[,<type>,<ec If <mode>=4, <type>, <eccnum> and <category> are
cnum>,<category>] specified, add an ECC number with assigned category:
OK
If <mode>=5, <type>, <eccnum> and <category> are
omitted, query all the ECC numbers and their categories:
+QECCNUM: 0,<eccnum1>,<category>[,…]
+QECCNUM: 1,<eccnum1>,<category>[,…]
+QECCNUM: 2,<eccnum1>,<category>[,…]
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 154 / 282

5G Module Series
+QECCNUM: 3,<eccnum1>,<category>[,…]
OK
If there is any error:
ERROR
Read Command Response
AT+QECCNUM? +QECCNUM: 0,<eccnum1>,<eccnum2>[,…]
+QECCNUM: 1,<eccnum1>,<eccnum2>[,…]
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<mode> Integer type. ECC number operation.
0 Query ECC number.
1 Add ECC number with default category.
2 Delete ECC number.
3 Reset ECC number list
4 Add ECC number with specified category.
5 Query all emergency call numbers and their categories.
<type> Integer type. ECC number type.
0 ECC number stored in module without (U)SIM card
1 ECC number stored in module with (U)SIM card
2 ECC number from network
3 ECC number from (U)SIM card
<category> Integer type. ECC number category.
0 Default
1 Police
2 Ambulance
4 Fire brigade
8 Marine guard
16 Mountain rescue
32 Manually initiated eCall
64 Automatically initiated eCall
<eccnum> String type. ECC number (e.g."110", "119").
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 155 / 282

5G Module Series
NOTE
1. Only the ECC numberstored in the module with/without the (U)SIM card can be modified.
2. If a number to be added into the type of ECC numbers with the (U)SIM card already exists in the
module, and has been obtained from the network and the (U)SIM card, it cannot be added.
3. The priority for reading ECC number list: ECC number from the network > ECC number from the
(U)SIM card >ECC number stored in the module with/without the (U)SIM card.
4. Executing AT+QECCNUM=<mode>[,<type>[,<eccnum1>[,<eccnum2>[,…[,<eccnumn>]]]]]
writes data to NVM. Please proceed with caution.
5. Executing AT+QECCNUM=<mode>[,<type>,<eccnum>,<category>] writes data to NVM. Please
proceed with caution.
Example
AT+QECCNUM=? //Query the supported ECC number operation mode.
+QECCNUM: (0-5)
OK
AT+QECCNUM? //Query ECC numbers with or without (U)SIM card.
+QECCNUM: 0,"911","112","00","08","110","999","118","119"
+QECCNUM: 1,"911","112"
OK
AT+QECCNUM=0,1 //Query ECC numbers stored in module with (U)SIM card.
+QECCNUM: 1,"911","112"
OK
AT+QECCNUM=1,1,"110","234"
//Add "110" and "234" to ECC numbers stored in module with (U)SIM card.
OK
AT+QECCNUM=0,1 //Query ECC numbers stored in module with (U)SIM card.
+QECCNUM: 1,"911","112","110","234"
OK
AT+QECCNUM=2,1,"110"
//Delete "110" from ECC numbers stored in module with (U)SIM card.
OK
AT+QECCNUM=0,1 //Query ECC numbers stored in module with (U)SIM card.
+QECCNUM: 1,"911","112","234"
OK
AT+QECCNUM=5 //Query all emergency call numbers and corresponding category.
+QECCNUM: 0,"911",0,"112",0,"00",0,"08",0,"110",0,"999",0,"118",0,"119",0
+QECCNUM: 1,"911",0,"112",0,"234",0
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 156 / 282

5G Module Series
+QECCNUM: 2,"110",1,"120",2,"119",4,"122",8,"999",16
+QECCNUM: 3,"112",0,"000",0,"08",0,"118",0,"122",0,"911",0,"999",0,"119",0,"120",0,"110",0
OK
AT+QECCNUM=4,1,"123",1
//Add ECC number "123" of the police category into ECC numbers stored in module with (U)SIM card.
OK
AT+QECCNUM=5 //Query all emergency call numbers and corresponding category.
+QECCNUM: 0,"911",0,"112",0,"00",0,"08",0,"110",0,"999",0,"118",0,"119",0
+QECCNUM: 1,"911",0,"112",0,"234",0,"123",1
+QECCNUM: 2,"110",1,"120",2,"119",4,"122",8,"999",16
+QECCNUM: 3,"112",0,"000",0,"08",0,"118",0,"122",0,"911",0,"999",0,"119",0,"120",0,"110",0
OK
AT+QECCNUM=3
//Reset the ECC number list, and such reset will take effect after the module is rebooted.
OK
6.17. AT^DSCI Indicate Call Status
This command indicates the call status.
AT^DSCI Indicate Call Status
Test Command Response
AT^DSCI=? ^DSCI: (list of supported <n>s)
OK
Read Command Response
AT^DSCI? ^DSCI: <n>
OK
Write Command Response
AT^DSCI=[<n>] OK
Characteristics -
Reference -
Parameter
<n> Integer type. Enable/disable URC of DSCI.
0 Disable
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 157 / 282

                                          5G Module Series
|       | 1  Enable  |     |     |
| ----- | ---------- | --- | --- |

NOTE

When the presentation of the DSCI at the TE is enabled, an URC is returned after the action:
^DSCI: <id>,<dir>,<stat>,<type>,<number>,<num_type>

Parameter
| <id>      |   Integer type. Call ID.         |     |     |
| --------- | -------------------------------- | --- | --- |
| <dir>     |   Integer type. Call direction.  |     |     |
|           | 0  Mobile originated call        |     |     |
|           | 1  Mobile terminated call        |     |     |
| <stat>    |   Integer type. Call state.      |     |     |
|           |   1  CALL_LOCAL_HOLD             |     |     |
|           |   2  CALL_ORIGINAL               |     |     |
|           |   3  CALL_CONNECT                |     |     |
|           |   4  CALL_INCOMING               |     |     |
|           |   5  CALL_WAITING                |     |     |
|           |   6  CALL_END                    |     |     |
|           |   7  CALL_ALERTING               |     |     |
|           |   8  CALL_REMOTE_HOLD            |     |     |
|           |   9  CALL_BOTH_HOLD              |     |     |
| <type>    |   Integer type. Call type.       |     |     |
|           |   0  Voice call                  |     |     |
|           |   1  PS call                     |     |     |
| <number>  |   String type. Phone number.     |     |     |
<num_type>   Integer type. Octet address type (See 3GPP TS 24.008 subclause 10.5.4.7 for details).
Usually, it has three value types:
|     | 129  |   Unknown type  |     |
| --- | ---- | --------------- | --- |
        145   International type (contains the character "+")
|       |   161   | National type  |     |
| ----- | ------- | -------------- | --- |
Example
//Dial a call.
| AT^DSCI=1                     |       |       |     //Enable DSCI.    |
| ----------------------------- | ----- | ----- | --------------------- |
| OK                            |       |       |                       |
| ATD10086;                     |       |       |     //Dial a number.  |
OK

| ^DSCI: 1,0,2,0,10086,129   |     |     |      //A call is originated.  |
| -------------------------- | --- | --- | ----------------------------- |

| ^DSCI: 1,0,7,0,10086,129   |     |     |     //The call is alerting.  |
| -------------------------- | --- | --- | ---------------------------- |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                158 / 282

5G Module Series
^DSCI: 1,0,3,0,10086,129 //The call is connected.
ATH
OK
^DSCI: 1,0,6,0,10086,129 //The call is ended.
//Incoming call.
RING
^DSCI: 1,1,4,0,13022100000,129 //A call is coming.
RING
^DSCI: 1,1,6,0,13022100000,129 //The call is ended.
NO CARRIER
6.18. AT+VTS Generate DTMF Tone
This command sends ASCII characters which cause MSC to transmit DTMF tones to a remote
subscriber. This command can only be operated in a voice call.
AT+VTS Generate DTMF Tone
Test Command Response
AT+VTS=? +VTS: (list of supported <DTMF_string>s),(list of supported
<duration>s)
OK
Write Command Response
AT+VTS=<DTMF_string>[,<duration>] OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Determined by the length of <DTMF_string> and
Maximum Response Time
<duration>.
Characteristics -
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 159 / 282

5G Module Series
Parameter
<DTMF_string> String type. ASCII characters in the set 0-9, #, *, A, B, C, D.
Maximal length: 31 bytes. When sending multiple tones at a time, the time
interval of two tones can be specified by AT+VTD.
<duration> Integer type. Tone duration in 1/10 seconds with tolerance. Range: 0–255.
If the duration is less than the minimum time specified by the network, the actual
duration will be the network specified time.
If this parameter is omitted, <duration> is specified by AT+VTD.
<err> Error code. For more details, see Chapter 13.5.
Example
ATD12345678900; //Dial a number.
OK
//Call connected.
AT+VTS="1" //The remote caller can hear the DTMF tone.
OK
AT+VTS="1234567890A" //Send multiple tones at a time.
OK
6.19. AT+VTD Set DTMF Tone Duration
This command sets the duration of DTMF tones. It can also set time interval of two tones when sending
multiple tones at a time.
AT+VTD Set DTMF Tone Duration
Test Command Response
AT+VTD=? +VTD: (list of supported <duration>s),(list of supported
<interval>s)
OK
Read Command Response
AT+VTD? +VTD: <duration>,<interval>
OK
Write Command Response
AT+VTD=<duration>[,<interval>] OK
If there is any error:
ERROR
Or
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 160 / 282

5G Module Series
+CME ERROR: <err>
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The parameters are not saved.
Reference 3GPP TS 27.007
Parameter
<duration> Integer type. Tone duration in 1/10 seconds with tolerance. Range: 0–255;
Default: 3. If the duration is less than the minimum time specified by the network,
the actual duration will be network specified time.
<interval> Integer type. Time interval of two tones in 1/10 seconds when sending multiple
tones at a time by AT+VTS. Range: 0–255. Default: 0.
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 161 / 282

5G Module Series
7
Phonebook Commands
7.1. AT+CNUM Get Subscriber Number
This command gets the subscriber’ own number(s) from the (U)SIM.
AT+CNUM Get Subscriber Number
Test Command Response
AT+CNUM=? OK
Execution Command Response
AT+CNUM [+CNUM: [<alpha>],<number>,<type>]
[…]
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<alpha> String type. Alphanumeric string associated with <number>. The used character set
should be the one selected with AT+CSCS.
<number> String type. Phone number of format specified by <type>.
<type> Integer type. Octet address type (See 3GPP TS 24.008 subclause 10.5.4.7).
129 Unknown type
145 International type (contains the character "+")
161 National type
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 162 / 282

                                          5G Module Series
7.2.  AT+CPBF  Find Phonebook Entry

This command searches phonebook entries starting with the given <findtext> string from the current
phonebook memory storage selected with AT+CPBS, and returns all found entries in alphanumeric order.
AT+CPBF  Find Phonebook Entry
Test Command  Response
AT+CPBF=?  +CPBF: <nlength>,<tlength>

OK
Write Command  Response
AT+CPBF=<findtext>  [+CPBF: <index>,<number>,<type>,<text>]
[…]

OK

If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time  Determined by phonebook entry storage
Characteristics  -
Reference  3GPP TS 27.007
Parameter
| <nlength>  |   Integer type. Maximum length of <number>.  |     |
| ---------- | -------------------------------------------- | --- |
| <tlength>  |   Integer type. Maximum length of <text>.    |     |
<findtext>    String type. Field of maximum length <tlength> in current TE character set specified
|       |   by AT+CSCS.  |     |
| ----- | -------------- | --- |
<index>     Integer type. Location number of phonebook memory storage.
| <number>  |   String type. Phone number of format <type>.  |     |
| --------- | ---------------------------------------------- | --- |
<type>      Integer type. Octet address type (See 3GPP TS 24.008 subclause 10.5.4.7 for
|       |   details).   |               |
| ----- | ------------- | ------------- |
|       |   129         | Unknown type  |
        145   International type (contains the character "+")
|       |   161   | National type  |
| ----- | ------- | -------------- |
<text>      Integer type. The field of maximum length <tlength> in current TE character set
|          |   specified by AT+CSCS.                            |     |
| -------- | -------------------------------------------------- | --- |
| <err>    |   Error code. For more details, see Chapter 13.5.  |     |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                163 / 282

5G Module Series
7.3. AT+CPBR Read Phonebook Entry
This command reads phonebook entries in location number range <index1> to <index2> from the
current phonebook memory storage selected with AT+CPBS. If <index2> is omitted, only location
<index1> is returned.
AT+CPBR Read Phonebook Entry
Test Command Response
AT+CPBR=? +CPBR: (list of supported <index>s),<nlength>,<tlength>
OK
Write Command Response
AT+CPBR=<index1>[,<index2>] +CPBR: <index1>,<number>,<type>,<text>
[…]
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time Determined by the phonebook entry storage.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<index> Integer type. Location number of phonebook memory storage.
<nlength> Integer type. Maximum length of field <number>.
<tlength> Integer type. Maximum length of field <text>.
<index1> Integer type. The first phonebook record to be read.
<index2> Integer type. The last phonebook record to be read.
<type> Integer type. Octet address type (See 3GPP TS 24.008 subclause 10.5.4.7 for
details).
129 Unknown type
145 International type (contains the character "+")
161 National type
<text> String type. Field of maximum length <tlength> in current TE character set specified
by AT+CSCS.
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 164 / 282

5G Module Series
7.4. AT+CPBS Select Phonebook Memory Storage
This command selects phonebook memory storage, which is used by other phonebook-related
commands. The Read Command returns currently selected phonebook memory storage, the number of
used locations and the number of total locations in the phonebook memory storage when supported by
manufacturer. The Test Command returns supported storage options as a compound value.
AT+CPBS Select Phonebook Memory Storage
Test Command Response
AT+CPBS=? +CPBS: (list of supported <storage>s)
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Read Command Response
AT+CPBS? +CPBS: <storage>[,<used>,<total>]
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Write Command Response
AT+CPBS=<storage> OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<storage> String type.
"SM" (U)SIM phonebook
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 165 / 282

5G Module Series
"DC" MT dialed call list (AT+CPBW may not be applicable to this storage)
"FD" (U)SIM fix dialing-phone book (AT+CPBW needs PIN2 authorization)
"LD" (U)SIM last dialing-phone book (AT+CPBW may not be applicable to this
storage)
"MC" MT missed (unanswered) call list (AT+CPBW may not be applicable to this
storage)
"ME" Mobile equipment phonebook
"RC" MT received call list (AT+CPBW may not be applicable to this storage)
"EN" (U)SIM (or MT) emergency number (AT+CPBW may not be applicable to this
storage)
"ON" (U)SIM own number (MSISDN) list
<used> Integer type. Count of used location(s) in selected phonebook memory storage.
<total> Integer type. Count of total location(s) in selected phonebook memory storage.
<err> Error code. For more details, see Chapter 13.5.
7.5. AT+CPBW Write Phonebook Entry
This command writes phonebook entry with location number <index> in the current phonebook memory
storage selected with AT+CPBS. It can also delete a phonebook entry with location number <index>.
AT+CPBW Write Phonebook Entry
Test Command Response
AT+CPBW=? +CPBW: (list of supported <index>s),<nlength>,(list of
supported <type>s),<tlength>
OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Write Command Response
AT+CPBW=[<index>][,<number>[,<ty OK
pe>[,<text>]]]
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 166 / 282

                                          5G Module Series
| Reference  |     |     | 3GPP TS 27.007  |
| ---------- | --- | --- | --------------- |
Parameter
<index>  Integer type. Location number of phonebook memory storage.
| <nlength>  | Integer type. Maximum length of field <number>.  |     |     |
| ---------- | ------------------------------------------------ | --- | --- |
| <tlength>  | Integer type. Maximum length of field <text>.    |     |     |
<number>    String type. Phone number. The format is determined by <type>.
<type>      Integer type. Octet address type (See 3GPP TS 24.008 subclause 10.5.4.).
|       |   129   | Unknown type  |     |
| ----- | ------- | ------------- | --- |
        145   International type (contains the character "+")
|       |   161   | National type  |     |
| ----- | ------- | -------------- | --- |
<text>      String type field of maximum length <tlength> in current TE character set specified by
|          |   AT+CSCS.                                         |     |     |
| -------- | -------------------------------------------------- | --- | --- |
| <err>    |   Error code. For more details, see Chapter 13.5.  |     |     |
Example
AT+CSCS="GSM"
OK
AT+CPBW=10,"15021012496",129,"QUECTEL"   //Write a new phonebook entry at location 10.
| OK     |       |       |         |
| ------ | ----- | ----- | ------- |
AT+CPBW=10                  //Delete an entry at location 10.
OK

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                167 / 282

5G Module Series
8
Short Message Service Commands
8.1. AT+CSMS Select Message Service
This command selects message service and queries the type of the message supported by MT.
AT+CSMS Select Message Service
Test Command Response
AT+CSMS=? +CSMS: (list of supported <service>s)
OK
Read Command Response
AT+CSMS? +CSMS: <service>,<mt>,<mo>,<bm>
OK
Write Command Response
AT+CSMS=<service> +CSMS: <mt>,<mo>,<bm>
OK
If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.005
Parameter
<service> Integer type. Messaging service type.
0 See 3GPP TS 23.040 and 3GPP TS 23.041 (the syntax of SMS AT commands is
compatible with 3GPP TS 27.005 Phase 2 version 4.7.0; Phase 2+ features which
do not require new command syntax can be supported, e.g., correct routing of
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 168 / 282

5G Module Series
messages with new Phase 2+ data coding schemes).
1 See 3GPP TS 23.040 and 3GPP TS 23.041 (the syntax of SMS AT commands is
compatible with 3GPP TS 27.005 Phase 2+ version; the requirement of <service>
setting 1 is mentioned under corresponding command descriptions).
<mt> Integer type. Mobile terminated message.
0 Type not supported
1 Type supported
<mo> Integer type. Mobile originated message.
0 Type not supported
1 Type supported
<bm> Integer type. Broadcast type message.
0 Type not supported
1 Type supported
<err> Error code. For more details, see Chapter 13.6.
Example
AT+CSMS=? //Test command.
+CSMS: (0,1)
OK
AT+CSMS=1 //Set messaging service type to 1.
+CSMS: 1,1,1
OK
AT+CSMS? //Read command.
+CSMS: 1,1,1,1
OK
8.2. AT+CMGF Set Message Format
This command specifies the input and output format of the short messages. <mode> indicates the
format of messages used with message send, list, read and write commands and URCs resulting from
received messages.
The format of messages can be either PDU mode (entire TP data units used) or text mode (headers and
body of messages given as separate parameters). Text mode uses the value of <chset> specified by
AT+CSCS to inform the character set to be used in the message body in the TA-TE interface.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 169 / 282

5G Module Series
AT+CMGF Set Message Format
Test Command Response
AT+CMGF=? +CMGF: (list of supported <mode>s)
OK
Read Command Response
AT+CMGF? +CMGF: <mode>
OK
Write Command/Execution Command Response
AT+CMGF[=<mode>] OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.005
Parameter
<mode> Integer type. Short message input and output format.
0 PDU mode
1 Text mode
8.3. AT+CSCA Set Service Center Address
The Write Command updates the SMSC address through which mobile originated SMSs are transmitted.
In text mode, the setting is used by Write Command. In PDU mode, setting is used by the same
command, but only when the length of the SMSC address is coded into <pdu> that equals zero.
AT+CSCA Set Service Center Address
Test Command Response
AT+CSCA=? OK
Read Command Response
AT+CSCA? +CSCA: <sca>,<tosca>
OK
Write Command Response
AT+CSCA=<sca>[,<tosca>] OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 170 / 282

5G Module Series
Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Reference 3GPP TS 27.005
Parameter
<sca> String type. Service center address. See 3GPP TS 24.011 RP SC address Address-
Value field; BCD numbers (or GSM 7-bit default alphabet characters) are converted
to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007).
<tosca> Integer type. Type of service center address in octet. See 3GPP TS 24.011 RP SC
address Type-of-Address.
<pdu> String type. Service center address in hexadecimal. In case of SMS: 3GPP TS
24.011 SC address followed by 3GPP TS 23.040 TPDU: ME/TA converts each octet
of TPDU into two IRA characters, with each octet represented by a pair of
hexadecimal numbers (e.g. octet with integer value 42 is presented to TE as two
characters 2A (IRA 50 and 65)).
Example
AT+CSCA="+8613800210500",145 //Set SMSC address.
OK
AT+CSCA? //Query SMSC address.
+CSCA: "+8613800210500",145
OK
8.4. AT+CPMS Select Message Memory Storage
This command selects memory storage to be used for reading, writing, etc.
AT+CPMS Select Message Memory Storage
Test Command Response
AT+CPMS=? +CPMS: (list of supported <mem1>s),(list of supported
<mem2>s),(list of supported <mem3>s)
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 171 / 282

                                          5G Module Series
Read Command  Response
AT+CPMS?  +CPMS: <mem1>,<used1>,<total1>,<mem2>,<used2>,<t
otal2>,<mem3>,<used3>,<total3>

OK
Write Command  Response
AT+CPMS=<mem1>[,<mem2>[,<mem +CPMS: <used1>,<total1>,<used2>,<total2>,<used3>,<to
3>]]  tal3>

OK

If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time  300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Reference  3GPP TS 27.005
Parameter
<mem1>    String type. Message to be read and deleted from this memory storage.
|       |   "SM"  |   (U)SIM message storage              |
| ----- | ------- | ------------------------------------- |
|       |   "ME"  |   Mobile equipment message storage    |
|       |   "MT"  |   Same as "ME" storage                |
|       |   "SR"  |   SMS status report storage location  |
<mem2>    String type. Message to be written and sent to this memory storage.
|       |   "SM"  |   (U)SIM message storage              |
| ----- | ------- | ------------------------------------- |
|       |   "ME"  |   Mobile equipment message storage    |
|       |   "MT"  |   Same as "ME" storage                |
|       |   "SR"  |   SMS status report storage location  |
<mem3>  String type. Received message to be placed in this memory storage if routing to PC is
not set (see AT+CNMI).
|       |   "SM"  |   (U)SIM message storage              |
| ----- | ------- | ------------------------------------- |
|       |   "ME"  |   Mobile equipment message storage    |
|       |   "MT"  |   Same as "ME" storage                |
|       |   "SR"  |   SMS status report storage location  |
<used>     Integer type. Number of current message(s) in <memn>.
<total>      Integer type. Number of total message(s) that can be stored in <memn>.
| <err>    |   Error code. For more details, see Chapter 13.6.  |     |
| -------- | -------------------------------------------------- | --- |

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                172 / 282

5G Module Series
NOTE
Executing AT+CPMS=<mem1>[,<mem2>[,<mem3>]] writes data to NVM. Please proceed with
caution.
Example
AT+CPMS? //Query the current SMS message storage.
+CPMS: "ME",0,255,"ME",0,255,"ME",0,255
OK
AT+CPMS="SM","SM","SM" //Set SMS message storage to "SM".
+CPMS: 0,50,0,50,0,50
OK
AT+CPMS? //Query the current SMS message storage.
+CPMS: "SM",0,50,"SM",0,50,"SM",0,50
OK
8.5. AT+CMGD Delete Message
This command deletes short message from the preferred message storage <mem1> location <index>. If
<delflag> is presented and not set to 0, ME ignores <index> and follows the rules of <delflag> shown
below.
AT+CMGD Delete Messages
Test Command Response
AT+CMGD=? +CMGD: (list of supported <index>s),(list of supported
<delflag>s)
OK
Write Command Response
AT+CMGD=<index>[,<delflag>] OK
If there is any error:
ERROR
Or
+CMS ERROR:<err>
Maximum Response Time 300 ms
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 173 / 282

5G Module Series
Characteristics -
Reference
3GPP TS 27.005
Parameter
<index> Integer type. Location number supported by the memory storage.
<delflag> Integer type. Flag indicating message deletion request.
0 Delete the message specified in <index>
1 Delete all read messages from <mem1>
2 Delete all read messages from <mem1> and sent mobile originated messages
3 Delete all read messages from <mem1>, and sent unsent mobile originated
messages
4 Delete all messages from <mem1>
<mem1> String type. Messages to be read and deleted from the memory storage.
"SM" (U)SIM message storage
"ME" Mobile equipment message storage
"MT" Same as "ME" storage
"SR" SMS status report storage location
<err> Error code. For more details, see Chapter 13.6.
NOTE
Executing AT+CMGD=<index>[,<delflag>] writes data to NVM. Please proceed with caution.
Example
AT+CMGD=1 //Delete the message specified in location 1.
OK
AT+CMGD=1,4 //Delete all messages from the memory storage.
OK
8.6. AT+CMGL Read Message by Status
This command returns message(s) with status parameter <stat> from preferred message storage
<mem1> to TE. If the status of the message is "REC UNREAD", the status in the storage changes to
"REC READ". When executing AT+CMGL without status parameter <stat>, it reports the list of SMS with
"REC UNREAD" status.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 174 / 282

5G Module Series
AT+CMGL Read Message by Status
Test Command Response
AT+CMGL=? +CMGL: (list of supported <stat>s)
OK
Write Command Response
AT+CMGL[=<stat>] If in text mode (AT+CMGF=1) and the command is executed
successfully:
For SMS-SUBMITs and/or SMS-DELIVERs:
+CMGL: <index>,<stat>,<oa/da>,[<alpha>],[<scts>][,<too
a/toda>,<length>]<CR><LF><data>[<CR><LF>]
[...]
For SMS-STATUS-REPORTs:
+CMGL: <index>,<stat>,<fo>,<mr>,[<ra>],[<tora>],<scts>,
<dt>,<st>[<CR><LF>]
[...]
For SMS-COMMANDs:
+CMGL: <index>,<stat>,<fo>,<ct>[<CR><LF>]
[…]
For CBM storage:
+CMGL: <index>,<stat>,<sn>,<mid>,<page>,<pages><C
R><LF><data>[<CR><LF>]
[...]
OK
If in PDU mode (AT+CMGF=0) and the command is
executed successfully:
+CMGL: <index>,<stat>,[<alpha>],<length><CR><LF><p
du>[<CR><LF>]
[...]
OK
If there is any error:
ERROR
Or
+CMS ERROR: <err>
The response time of <stat> operation depends on the
Maximum Response Time
storage of listed messages.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 175 / 282

                                          5G Module Series
The maximum response time is 300 ms.
| Characteristics  |     |     | -               |
| ---------------- | --- | --- | --------------- |
| Reference        |     |     | 3GPP TS 27.005  |
Parameter
<mem1>    String type. Messages to be read and deleted from the memory storage.
|       |   "SM"  |   (U)SIM message storage              |     |
| ----- | ------- | ------------------------------------- | --- |
|       |   "ME"  |   Mobile equipment message storage    |     |
|       |   "MT"  |   Same as "ME" storage                |     |
|       |   "SR"  |   SMS status report storage location  |     |
<stat>      Integer type (PDU mode), or string type (text mode). Message status in the storage.
|         |   In text mode:   |       |                          |
| ------- | ----------------- | ----- | ------------------------ |
|         |     "REC UNREAD"  |       | Received unread message  |
|         |     "REC READ"    |       | Received read message    |
|         |     "STO UNSENT"  |       | Stored unsent message    |
|         |     "STO SENT"    |       | Stored sent message      |
|         |     "ALL"         |       | All messages             |
|         |     In PDU mode:  |       |                          |
|         |     0             |       | Received unread message  |
|         |     1             |       | Received read message    |
|         |     2             |       | Stored unsent message    |
|         |     3             |       | Stored sent message      |
|         |     4             |       | All messages             |
<index>     Integer type. Location number supported by the memory storage.
<da>             String  type.  Destination  address.  See  3GPP  TS  23.040  TP-Destination-Address
  Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
  converted to characters of the currently selected TE character set (see AT+CSCS in
  3GPP TS 27.007). The type of address is given by <toda>.
<oa>      String  type.  Originating  address.  See  3GPP  TS  23.040  TP-Originating-Address
Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007). The type of address is given by <tooa>.
<alpha>          String type. Alphanumeric representation of <da> or <oa> corresponding to the entry
found in MT phonebook. Implementation of this feature is manufacturer specified. The
used  character  set  should  be  the  one  selected  with  AT+CSCS  (see
3GPP TS 27.007).
<scts>       String type. Service center time stamp. See 3GPP TS 23.040 TP-Service-Centre-
Time-Stamp (format see <dt> in Chapter 8.7).
<toda>         Integer  type.  Type  of  destination  address  in  octet.  See  3GPP  TS  24.011  TP-
Destination-Address Type-of-Address.
<tooa>      Integer  type.  Type  of  originating  address  in  octet.  See  3GPP  TS  24.011  TP-
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                176 / 282

5G Module Series
Originating-Address Type-of-Address.
<length> Integer type. Message length. Unit: byte.
In text mode (AT+CMGF=1): length of the message body <data> in characters.
In PDU mode (AT+CMGF=0): length of the actual TPDU in octet (i.e. the RP layer
SMSC address octets are not counted in the length).
<data> In case of SMS: 3GPP TS 23.040 TP-User-Data in text mode responses; Format:
If <dcs> indicates that 3GPP TS 23.038 GSM 7-bit default alphabet is used and <fo>
indicates that 3GPP TS 23.040 TP-User-Data-Header-Indication is not set.
If TE character set other than "HEX" (see AT+CSCS in 3GPP TS 27.007): ME/TA
converts GSM alphabet into current TE character set according to rules in
3GPP TS 27.005 Annex A.
If TE character set is "HEX": ME/TA converts each 7-bit character of GSM 7-bit
default alphabet into two IRA character long hexadecimal number (e.g. character 
(GSM 7-bit default alphabet 23) presented as 17 (IRA 49 and 55)).
If <dcs> indicates that 8-bit or UCS2 data coding scheme is used, or <fo> indicates
that 3GPP TS 23.040 TP-User-Data-Header-Indication is set: ME/TA converts each
8-bit octet into two IRA characters, with each octet represented by a pair of
hexadecimal numbers (e.g. octet with integer value 42 is presented to TE as two
characters 2A (IRA 50 and 65)).
In the case CBS: 3GPP TS 23.041 CBM Content of Message in text mode responses;
Format:
If <dcs> indicates that 3GPP TS 23.038 GSM 7-bit default alphabet is used:
If TE character set other than "HEX" (see AT+CSCS in 3GPP TS27.007): ME/TA
converts GSM alphabet into current TE character set according to rules of Annex A in
3GPP TS 27.005.
If TE character set is "HEX": ME/TA converts each 7-bit character of the GSM 7-bit
default alphabet into two IRA characters, with each octet represented by a pair of
hexadecimal numbers.
<fo> Integer type. It depends on the command or result code: the first octet of
3GPP TS 23.040 SMS-DELIVER, SMS-SUBMIT (default value 17), SMS-STATUS-
REPORT, or SMS-COMMAND (default value 2).
<mr> Integer type. Message reference. See 3GPP TS 23.040 TP-Message-Reference.
<ra> String type. Recipient address. See 3GPP TS 23.040 TP-Recipient-Address Address-
Value field; BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see command
AT+CSCS in 3GPP TS 27.007).
<tora> Integer type. Type of recipient address in octet. See 3GPP TS 24.011 TP-Recipient-
Address Type-of-Address.
<dt> String type. 3GPP TS 23.040 TP-Discharge-Time. Format: "yy/MM/dd,hh:mm:ss±zz",
where characters indicate year (two last digits), month, day, hour, minutes, seconds
and time zone. E.g., 6th of May 1994, 22:10:00 GMT+2 hours equals
"94/05/06,22:10:00+08".
<st> Integer type. See 3GPP TS 23.040 TP-Status.
<ct> Integer type. Default value: 0. See 3GPP TS 23.040 TP-Command-Type.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 177 / 282

5G Module Series
<sn> Integer type. See 3GPP TS 23.041 CBM Serial Number.
<mid> Integer type. See 3GPP TS 23.041 CBM Message Identifier.
<page> Integer type. See 3GPP TS 23.041 CBM Page Parameter bits 4–7.
<pages> Integer type. See 3GPP TS 23.041 CBM Page Parameter bits 0–3.
<pdu> String type. Service center address in hexadecimal. In case of SMS: 3GPP TS 24.011
SC address followed by 3GPP TS 23.040 TPDU: ME/TA converts each octet of TP
data unit into two IRA character long hexadecimal number (e.g. octet with integer
value 42 is presented to TE as two characters 2A (IRA 50 and 65)).
<err> Error code. For more details, see Chapter 13.6.
Example
AT+CMGF=1 //Set SMS format to text mode.
OK
AT+CMGL="ALL" //List all messages from message storage.
+CMGL: 1,"STO UNSENT","",,
<This is a test from Quectel>
+CMGL: 2,"STO UNSENT","",,
<This is a test from Quectel>
OK
8.7. AT+CMGR Read Message by Index
This command returns SMS message with location value <index> from message storage <mem1> to
the TE. If status of the message is "REC UNREAD", status in the storage changes to "REC READ".
AT+CMGR Read Message by Index
Test Command Response
AT+CMGR=? OK
Write Command Response
AT+CMGR=<index> If in text mode (AT+CMGF=1) and the command is executed
successfully:
For SMS-DELIVER:
+CMGR: <stat>,<oa>,[<alpha>],<scts>[,<tooa>,<fo>,<pid>,<d
cs>,<sca>,<tosca>,<length>]<CR><LF><data>
OK
For SMS-SUBMIT:
+CMGR: <stat>,<da>,[<alpha>][,<toda>,<fo>,<pid>,<dcs>,[<v
p>],<sca>,<tosca>,<length>]<CR><LF><data>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 178 / 282

5G Module Series
OK
For SMS-STATUS-REPORTs:
+CMGR: <stat>,<fo>,<mr>,[<ra>],[<tora>],<scts>,<dt>,<st>
OK
For SMS-COMMANDs:
+CMGR: <stat>,<fo>,<ct>[,<pid>,[<mn>],[<da>],[<toda>],<len
gth><CR><LF><cdata>]
OK
For CBM storage:
+CMGR: <stat>,<sn>,<mid>,<dcs>,<page>,<pages><CR><L
F><data>
OK
If in PDU mode (AT+CMGF=0) and command is executed
successfully:
+CMGR: <stat>,[<alpha>],<length><CR><LF><pdu>
OK
If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time Depends on the length of message content.
Characteristics -
Reference 3GPP TS 27.005
Parameter
<index> Integer type. Location number supported by the memory storage.
<stat> Integer type (PDU mode), or string type (text mode). Message status in the storage.
In text mode:
"REC UNREAD" Received unread message
"REC READ" Received read message
"STO UNSENT" Stored unsent message
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 179 / 282

                                          5G Module Series
|       |   "STO SENT"              | Stored sent message          |
| ----- | ------------------------- | ---------------------------- |
|       |   "ALL"                   |     All messages             |
|       |   In PDU mode:            |                              |
|       |   0                       |     Received unread message  |
|       |   1                       |     Received read message    |
|       |   2                       |     Stored unsent message    |
|       |   3                       |     Stored sent message      |
|       |   4                       |     All messages             |
<oa>          String  type.  Originating  address.  See  3GPP  TS  23.040
TP-Originating-Address  Address-Value  field.  BCD  numbers  (or  GSM  7-bit  default
alphabet characters) are converted to characters of the currently selected TE character
set (see AT+CSCS in 3GPP TS 27.007). The type of address is given by <tooa>.
<alpha>  String type. Alphanumeric representation of <da> or <oa> corresponding to the entry
found in MT phonebook. Implementation of this feature is manufacturer specified. The
used character set should be the one selected with AT+CSCS (see 3GPP TS 27.007).
| <scts>      | String type. Service center time stamp.   |     |
| ----------- | ----------------------------------------- | --- |
  See 3GPP TS 23.040 TP-Service-Centre-Time-Stamp (see <dt>).
| <tooa>          | Integer type. Type of originating address in octet.   |     |
| --------------- | ----------------------------------------------------- | --- |
  See 3GPP TS 24.011 TP-Originating-Address Type-of-Address.
<fo>  Integer type in octet. It depends on the command or result code: the first octet of
3GPP  TS  23.040  SMS-DELIVER,  SMS-SUBMIT  (default  value  17),
SMS-STATUS-REPORT, or SMS-COMMAND.
| <pid>  | Integer type. Protocol identifier. Default value: 0.   |     |
| ------ | ------------------------------------------------------ | --- |
|        | See 3GPP TS 23.040 TP-Protocol-Identifier.             |     |
<dcs>  Integer  type.  Data  coding  scheme.  It  depends  on  the  command  or  result  code:
3GPP TS 23.038 SMS Data Coding Scheme (default value 0), or Cell Broadcast Data
Coding Scheme.
<sca>           String  type.  Service  center  address.  See  3GPP  TS  24.011  RP  SC  address
Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007). The type of address is given by <tosca>.
<tosca>         Integer type. Type of service center address in octet. See 3GPP TS 24.011 RP SC
address Type-of-Address.
| <length>     | Integer type. Message length. Unit: byte.   |     |
| ------------ | ------------------------------------------- | --- |
  In text mode (AT+CMGF=1): length of the message body <data> (or <cdata>) in
characters.
  In PDU mode (AT+CMGF=0): length of the actual TPDU in octet (i.e. the RP layer
SMSC address octets are not counted in the length).
| <data>          | String type. Text of short message.   |     |
| --------------- | ------------------------------------- | --- |
<da>  String  type.  Destination  address.  See  3GPP  TS  23.040  TP-Destination-Address
Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007). The type of address is given by <toda>.
<toda>  Integer  type.  Type  of  destination  address  in  octet.  See  3GPP  TS  24.011
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                180 / 282

5G Module Series
TP-Destination-Address Type-of-Address.
<vp> Integer type or time-string type. Validity period. It depends on SMS-SUBMIT <fo>
setting: 3GPP TS 23.040 TP-Validity-Period (format see <dt>).
<mr> Integer type. Message reference. See 3GPP TS 23.040 TP-Message-Reference.
<ra> String type. Recipient address. See 3GPP TS 23.040 TP-Recipient-Address
Address-Value field. BCD numbers (or GSM default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS).
The type of address is given by <tora>.
<tora> Integer type. Type of recipient address in octet. See 3GPP TS 24.011
TP-Recipient-Address Type-of-Address.
<dt> String type. 3GPP TS 23.040 TP-Discharge-Time. Format: "yy/MM/dd,hh:mm:ss±zz",
where characters indicate year (two last digits), month, day, hour, minutes, seconds
and time zone. E.g., 6th of May 1994, 22:10:00 GMT+2 hours equals
"94/05/06,22:10:00+08".
<st> Integer type. See 3GPP TS 23.040 TP-Status.
<ct> Integer type. See 3GPP TS 23.040 TP-Command-Type (default value 0).
<mn> Integer type. Message number. See 3GPP TS 23.040 TP-Message-Number.
<cdata> String type. See 3GPP TS 23.040 TP-Command-Data in text mode responses; ME/TA
converts each 8-bit octet into two IRA characters, with each octet represented by a
pair of hexadecimal numbers (e.g. octet with integer value 42 is presented to TE as
two characters 2A (IRA 50 and 65)).
<sn> Integer type. See 3GPP TS 23.041 CBM Serial Number.
<mid> Integer type. See 3GPP TS 23.041 CBM Message Identifier.
<page> Integer type. See 3GPP TS 23.041 CBM Page Parameter bits 4–7.
<pages> Integer type. See 3GPP TS 23.041 CBM Page Parameter bits 0–3.
<pdu> String type. Service center address in hexadecimal. In case of SMS: 3GPP TS 24.011
SC address followed by 3GPP TS 23.040 TPDU: ME/TA converts each octet of TP data
unit into two IRA characters, with each octet represented by a pair of hexadecimal
numbers (e.g. octet with integer value 42 is presented to TE as two characters 2A (IRA
50 and 65)).
<err> Error code. For more details, see Chapter 13.6.
Example
+CMTI: "SM",3 //A new message has been received and saved
AT+CSDH=1 //Show the values in result codes.
OK
AT+CMGR=3 //Read message.
+CMGR: "REC UNREAD","+8615021012496",,"13/12/13,15:06:37+32",145,4,0,0,"+861380021050
0",145,27
<This is a test from Quectel>
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 181 / 282

5G Module Series
8.8. AT+CMGS Send Message
This command sends a short message from TE to the network (SMS-SUBMIT). After invoking the Write
Command, wait for the prompt > and then write the message. After that, tap CTRL + Z to indicate the
ending of PDU and initiate message sending. Tap Esc to cancel the sending. Abortion is acknowledged
with OK, though the message will not be sent. On successful message delivery, the message reference
<mr> is returned to TE. The value can be used to identify the message upon unsolicited delivery of a
status report result code.
AT+CMGS Send Message
Test Command Response
AT+CMGS=? OK
Write Command Response
1) If in text mode (AT+CMGF=1): >
AT+CMGS=<da>[,<toda>] After > is returned, input the message to be sent. Tap CTRL
+ Z to send the message or tap Esc to cancel the sending.
2) If in PDU mode (AT+CMGF=0):
AT+CMGS=<length> If the message is sent successfully:
+CMGS: <mr>
OK
If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time 120 s, determined by the network.
Characteristics -
Reference 3GPP TS 27.005
Parameter
<da> String type. Destination address. See 3GPP TS 23.040 TP-Destination-Address
Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007). The type of address is given by <toda>.
<toda> Integer type. Type of destination address in octet. See 3GPP TS 24.011
TP-Destination-Address Type-of-Address.
<length> Message length.
In text mode (AT+CMGF=1): length of the message body in characters.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 182 / 282

5G Module Series
In PDU mode (AT+CMGF=0): length of the actual TPDU in octet (i.e. the RP layer
SMSC address octets are not counted in the length).
<mr> Integer type. Message reference. See 3GPP TS 23.040 TP-Message-Reference.
<err> Error code. For more details, see Chapter 13.6.
Example
AT+CMGF=1 //Set SMS message format to text mode.
OK
AT+CSCS="GSM" //Set character set to GSM used by TE.
OK
AT+CMGS="15021012496"
>This is a test from Quectel //Enter the message and tap CTRL + Z to send message.
+CMGS: 247
OK
8.9. AT+CMMS Send Multiple Messages
This command controls the continuity of the SMS relay protocol link. If the feature is enabled (and
supported by the network) multiple messages can be sent faster as the link is kept open.
AT+CMMS Send Multiple Messages
Test Command Response
AT+CMMS=? +CMMS: (list of supported <n>s)
OK
Read Command Response
AT+CMMS? +CMMS: <n>
OK
Write Command Response
AT+CMMS[=<n>] OK
If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time 120 s, determined by network.
Characteristics -
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 183 / 282

5G Module Series
Reference 3GPP TS 27.005
Parameter
<n> Integer type.
0 Feature disabled
1 Keep enabled until the time between the response of the latest command to be sent
(AT+CMGS, AT+CMSS, etc.) and the next command to be sent exceeds 1–5 seconds
(the exact value is up to ME implementation); then ME closes the link automatically
and switches <n> back to 0.
2 Feature enabled. If the time between the response of the latest command to be sent
and the next command to be sent exceeds 1–5 seconds (the exact value is up to
ME implementation), ME closes the link but MT does not automatically switch <n>
back to 0.
<err> Error code. For more details, see Chapter 13.6.
NOTE
Once the Read Command is executed, a delay of 5–10 seconds is required before issuing the Write
Command. Otherwise +CMS ERROR: 500 may be returned.
8.10. AT+CMGW Write Message to Memory Storage
This command stores short messages from TE to memory storage <mem2>, and then the location
<index> of the stored message is returned. Message status will be set to "STO UNSENT" by default; but
<stat> also allows entering other status values.
The syntax of input text is the same as the one specified in AT+CMGS Write Command.
AT+CMGW Write Message to Memory Storage
Test Command Response
AT+CMGW=? OK
Write Command Response
1) If in text mode (AT+CMGF=1): >
AT+CMGW=<oa/da>[,<tooa/toda>[,<s After > is returned, input the message to be sent. Tap CTRL
tat>]] + Z to send the message or tap Esc to cancel the sending.
2) If in PDU mode (AT+CMGF=0): If message writing is successful:
AT+CMGW=<length>[,<stat>] +CMGW: <index>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 184 / 282

                                          5G Module Series
OK

If there is any error:
ERROR
Or
+CMS ERROR: <err>
| Maximum Response Time  |     |     | 300 ms          |
| ---------------------- | --- | --- | --------------- |
| Characteristics        |     |     | -               |
| Reference              |     |     | 3GPP TS 27.005  |
Parameter
<da>  String  type.  Destination  address.  See  3GPP  TS  23.040  TP-Destination-Address
Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007). The type of address is given by <toda>.
<oa>  String  type.  Originating  address.  See  3GPP  TS  23.040  TP-Originating-Address
Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007). The type of address given by <tooa>.
<tooa>  Integer type. Type of originating address in octet. See 3GPP TS 24.011 TP-Originating-
Address Type-of-Address.
<stat>  Integer type (PDU mode), or string type (text mode). Message status in the storage.
|       | In text mode:   |       |                          |
| ----- | --------------- | ----- | ------------------------ |
|       | "REC UNREAD"    |       | Received unread message  |
|       | "REC READ"      |       | Received read message    |
|       | "STO UNSENT"    |       | Stored unsent message    |
|       | "STO SENT"      |       | Stored sent message      |
|       | "ALL"           |       | All messages             |
|       |   In PDU mode:  |       |                          |
|       |   0             |       | Received unread message  |
|       |   1             |       | Received read message    |
|       |   2             |       | Stored unsent message    |
|       |   3             |       | Stored sent message      |
|       |  4              |       | All messages             |
<toda>  Integer  type.  Type  of  destination  address  in  octet.  See  3GPP  TS  24.011  TP-
Destination-Address Type-of-Address.
| <length>  | Integer type. Message length. Unit: byte.   |     |     |
| --------- | ------------------------------------------- | --- | --- |
  In text mode (AT+CMGF=1): length of the message body in characters.
  In PDU mode (AT+CMGF=0): length of the actual TPDU in octet (i.e. the RP layer
SMSC address octets are not counted in the length).
<index>  Integer type. Index of message in the memory storage <mem2>.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                185 / 282

5G Module Series
<mem2> String type. Message to be written and sent to the memory storage.
"SM" (U)SIM message storage
"ME" Mobile equipment message storage
"MT" Same as "ME" storage
"SR" SMS status report storage location
<err> Error code. For more details, see Chapter 13.6.
NOTE
1. Executing AT+CMGW writes data to NVM. Please proceed with caution.
2. For details about <pdu>, See Chapter 8.2.
Example
AT+CMGF=1 //Set SMS message format to text mode.
OK
AT+CSCS="GSM" //Set character set to GSM used by TE.
OK
AT+CMGW="15021012496"
> This is a test from Quectel //Enter message in text and tap CTRL + Z to write message.
+CMGW: 4
OK
AT+CMGF=0 //Set SMS message format to PDU mode.
OK
AT+CMGW=18
> 0051FF00000008000A0500030002016D4B8BD5
+CMGW: 5
OK
8.11. AT+CMSS Send Messages from Memory Storage
This command sends a message with location parameter <index> from message storage <mem2> to
the network. If a new recipient address <da> is given for SMS-SUBMIT, it should be used instead of the
old one contained in the message stored in the memory storage. Reference value <mr> is returned to
TE on successful message delivery. The value can be used to identify the message upon unsolicited
delivery status report result code.
AT+CMSS Send Messages from Memory Storage
Test Command Response
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 186 / 282

5G Module Series
AT+CMSS=? OK
Write Command Response
AT+CMSS=<index>[,<da>[,<toda>]] If in text mode (AT+CMGF=1) and the message is sent
successfully:
+CMSS: <mr>[,<scts>]
OK
If in PDU mode (AT+CMGF=0) and the message is sent
successfully:
+CMSS: <mr>[,<ackpdu>]
OK
If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time 120 s, determined by network.
Characteristics -
Reference 3GPP TS 27.005
Parameter
<index> Integer type. Location number supported by the memory storage.
<da> String type. Destination Address. See 3GPP TS 23.040 TP-Destination-Address
Address-Value field. BCD numbers (or GSM 7-bit default alphabet characters) are
converted to characters of the currently selected TE character set (see AT+CSCS in
3GPP TS 27.007). The type of address is given by <toda>.
<toda> Integer type. Type of destination address in octet. See 3GPP TS 24.011
TP-Destination-Address Type-of-Address.
<mr> Integer type. Message reference. See 3GPP TS 23.040 TP-Message-Reference.
<scts> String type. Service center time stamp. See 3GPP TS 23.040
TP-Service-Centre-Time-Stamp (format see <dt> in Chapter 8.7).
<ackpdu> String type. 3GPP TS 23.040 RP-User-Data element of RP-ACK PDU. Format is the
same with <pdu> in case of SMS, but without 3GPP TS 24.011 SC address field.
<mem2> String type. Messages to be written and sent to the memory storage.
"SM" (U)SIM message storage
"ME" Mobile equipment message storage
"MT" Same as "ME" storage
"SR" SMS status report storage location
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 187 / 282

5G Module Series
<err> Error code. For more details, see Chapter 13.6.
Example
AT+CMGF=1 //Set SMS message format to text mode.
OK
AT+CSCS="GSM" //Set character set to GSM used by TE.
OK
AT+CMGW="15021012496"
>Hello //Enter message in text and tap CTRL + Z to send message.
+CMGW: 4
OK
AT+CMSS=4 //Send message of index 4 from memory storage.
+CMSS: 54
OK
8.12. AT+CNMA New Message Acknowledgement
This command confirms successful receipt of a new message (SMS-DELIVER or SMS-STATUS-
REPORT) routed directly to TE. If UE does not receive acknowledgement within required time (network
timeout), it will send an RP-ERROR message to the network. UE will automatically disable routing to TE
by setting both <mt> and <ds> values of AT+CNMI to 0.
AT+CNMA New Message Acknowledgement
Test Command Response
AT+CNMA=? If in text mode (AT+CMGF=1):
OK
If in PDU mode (AT+CMGF=0):
+CNMA: (list of supported <n>s)
OK
Execution Command Response
If in text mode (AT+CMGF=1): OK
AT+CNMA
If there is any error:
ERROR
Or
+CMS ERROR: <err>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 188 / 282

5G Module Series
Write Command Response
If in PDU mode (AT+CMGF=0): OK
AT+CNMA=<n>[,<length>[<CR>
PDU is given<Ctrl + Z/Esc>]] If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.005
Parameter
<n> Integer type. Parameter required only for PDU mode
0 Command operates similarly as in text mode
1 Send positive (RP-ACK) acknowledgement to the network. Accepted only in PDU
mode.
2 Send negative (RP-ERROR) acknowledgement to the network. Accepted only in PDU
mode.
<length> Integer type. Message length. Unit: byte. Length of actual TPDU in octets (i.e. the RP layer
SMSC address octets are not counted in the length) in PDU mode (AT+CMGF=0).
<err> Error code. For more details, see Chapter 13.6.
NOTE
The Execution and Write Commands are only used when <service> of AT+CSMS equals 1 (phase 2+)
and an appropriate URC has been issued by MT, i.e.:
1) +CMT when <mt>=2 in AT+CNMI, i.e. incoming message classes 0, 1, 3 and none.
2) +CMT when <mt>=3 in AT+CNMI, i.e. incoming message classes 0 and 3.
3) +CDS when <ds>=1 in AT+CNMI.
Example
AT+CSMS=1
OK
AT+CNMI=1,2,0,0,0
OK
AT+CMGF=1 //Set SMS message format to text mode.
OK
AT+CSDH=1 //Show the values in result codes
OK
+CMT: "+8615021012496",,"13/03/18,17:07:21+32",145,4,0,0,"+8613800551500",145,28
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 189 / 282

                                          5G Module Series
This is a test from Quectel.   //Short message is outputted directly upon incoming SMS.
| AT+CNMA  |       |   //Send ACK to the network.  |     |     |     |     |
| -------- | ----- | ----------------------------- | --- | --- | --- | --- |
OK
| AT+CNMA  |       |   //Send ACK to the network.  |     |     |     |     |
| -------- | ----- | ----------------------------- | --- | --- | --- | --- |
+CMS ERROR: 340      //An error is returned on the second attempt; only one ACK is required.

8.13. AT+CNMI  Set New Message Indication

This command selects how the received new message from the network are indicated to TE when TE is
active, e.g., DTR is at low level (ON). If TE is inactive (e.g., DTR is at high level (OFF)), the message
should be received as specified in 3GPP TS 23.038.
AT+CNMI  Set New Message Indication
| Test Command  |     |     | Response  |     |     |     |
| ------------- | --- | --- | --------- | --- | --- | --- |
AT+CNMI=?  +CNMI:  (list  of  supported  <mode>s),(list  of  supported
|     |     |     | <mt>s),(list  | of  supported  | <bm>s),(list  | of  supported  |
| --- | --- | --- | ------------- | -------------- | ------------- | -------------- |
<ds>s),(list of supported <bfr>s)

OK
| Read Command  |     |     | Response                            |     |     |     |
| ------------- | --- | --- | ----------------------------------- | --- | --- | --- |
| AT+CNMI?      |     |     | +CNMI: <mode>,<mt>,<bm>,<ds>,<bfr>  |     |     |     |

OK
| Write Command                   |     |     | Response  |     |     |     |
| ------------------------------- | --- | --- | --------- | --- | --- | --- |
| AT+CNMI=[<mode>[,<mt>[,<bm>[,<d |     |     | OK        |     |     |     |
| s>[,<bfr>]]]]]                  |     |     |           |     |     |     |
If there is any error:
ERROR
Or
+CMS ERROR: <err>
| Maximum Response Time  |     |     | 300 ms  |     |     |     |
| ---------------------- | --- | --- | ------- | --- | --- | --- |
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
| Reference  |     |     | 3GPP TS 27.005  |     |     |     |
| ---------- | --- | --- | --------------- | --- | --- | --- |
Parameter
| <mode>  | Integer type.   |     |     |     |     |     |
| ------- | --------------- | --- | --- | --- | --- | --- |
      0  Buffer URC in MT. If MT result code buffer is full, the indication can be buffered in
        some other place or the oldest indication may be discarded and replaced with the new
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                190 / 282

5G Module Series
received indication.
1 Discard the indication and reject new received message URC when MT-TE link is
reserved (e.g. in data mode). Otherwise, forward directly to TE.
2 Buffer URC in MT when MT-TE link is reserved (e.g. in data mode) and flush URC to
TE after reservation. Otherwise forward directly to TE.
<mt> Integer type. Rule for storing received message, which depends on its data coding
scheme (see 3GPPTS 23.038) and preferred memory storage (AT+CPMS) setting.
0 No SMS-DELIVER indication is routed to TE.
1 If SMS-DELIVER is stored into ME/TA, the indication of the memory location is
routed to TE by URC: +CMTI: <mem>,<index>.
2 SMS-DELIVER (except Class 2) is routed directly to TE by URC: +CMT:
[<alpha>],<length><CR><LF><pdu> (PDU mode enabled) or URC +CMT:
<oa>,[<alpha>],<scts>[,<tooa>,<fo>,<pid>,<dcs>,<sca>,<tosca>,
<length>]<CR><LF><data> (text mode enabled; about the parameters in italics,
see AT+CSDH). Class 2 message results in indication as defined in <mt>=1.
3 Class 3 SMS-DELIVER is routed directly to TE by URC defined in <mt>=2. Message
of other classes results in indication as defined in <mt>=1.
<bm> Integer type. Rule for storing received CBM, which depends on its data coding scheme
(see 3GPP TS 23.038) and the setting of selected CBM types (AT+CSCB).
0 No CBM indication is routed to TE.
2 New CBM is routed directly to TE by URC: +CBM: <length><CR><LF><pdu>
(PDU mode enabled);
or URC +CBM: <sn>,<mid>,<dcs>,<page>,<pages><CR><LF><data>
(text mode enabled).
<ds> Integer type.
0 No SMS-STATUS-REPORT is routed to TE.
1 SMS-STATUS-REPORT is routed to TE by URC:
+CDS: <length><CR><LF><pdu> (PDU mode) or
+CDS: <fo>,<mr>,[<ra>],[<tora>],<scts>,<dt>,<st> (text mode)
2 If SMS-STATUS-REPORT is stored into ME/TA, the indication of the memory
location is routed to TE by URC: +CDSI: <mem>,<index>
<bfr> Integer type.
0 TA buffer of URC defined within this command is flushed to TE when <mode> is 1 or
2 (response OK shall be given before flushing).
1 TA buffer of URC defined within this command is cleared when <mode> is 1 or 2.
<err> Error code. For more details, see Chapter 13.6.
NOTE
1. URC +CMTI: <mem>,<index> A new message has been received.
2. URC +CMT: [<alpha>],<length><CR><LF><pdu> A short message is output directly.
3. URC +CBM: <length><CR><LF><pdu> A cell broadcast message is output directly.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 191 / 282

5G Module Series
Example
AT+CMGF=1 //Set SMS message format to text mode.
OK
AT+CSCS="GSM" //Set character set to GSM, which is used by TE.
OK
AT+CNMI=1,2,0,1,0 //Set SMS-DELIVER to be routed directly to TE.
OK
AT+CSDH=1 //Show text mode parameter.
OK
+CMT: "+8615021012496",,"13/03/18,17:07:21+32",145,4,0,0,"+8613800551500",145,28
This is a test from Quectel. //Short message is output directly when an SMS is incoming.
8.14. AT+CSCB Select Cell Broadcast Message Type
This command selects the types of CBMs to be received by the ME.
AT+CSCB Select Cell Broadcast Message Types
Test Command Response
AT+CSCB=? +CSCB: (list of supported <mode>s)
OK
Read Command Response
AT+CSCB? +CSCB: <mode>,<mids>,<dcss>
OK
Write Command Response
AT+CSCB=<mode>[,mids>[,<dcss>]] OK
If there is any error:
ERROR
Or
+CMS ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.005
Parameter
<mode> Integer type.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 192 / 282

5G Module Series
0 Message type specified in <mids> and <dcss> is accepted
1 Message type specified in <mids> and <dcss> is not accepted
<mids> String type. All different possible combinations of CBM message identifiers (see <mid>)
(default: empty string), e.g. "0,1,5,320–478,922".
<dcss> String type. All different possible combinations of CBM data coding schemes (see <dcs>)
(default: empty string), e.g. "0–3,5".
<err> Error code. For more details, see Chapter 13.6.
8.15. AT+CSDH Show Text Mode Parameter
This command controls whether to show detailed header information in text mode result codes.
AT+CSDH Show Text Mode Parameter
Test Command Response
AT+CSDH=? +CSDH: (list of supported <show>s)
OK
Read Command Response
AT+CSDH? +CSDH: <show>
OK
Write Command Response
AT+CSDH=[<show>] OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.005
Parameter
<show> Integer type.
0 Do not show detailed header information: for SMS-DELIVERs and SMS-
SUBMITs in text mode, URCs +CSCA and +CSMP do not contain <sca>,
<tosca>, <fo>, <vp>, <pid> or <dcs> and URCs +CMT, +CMGL and +CMGR
do not contain <length>, <toda> or <tooa>.
1 Show the values in result codes
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 193 / 282

5G Module Series
Example
AT+CSDH=0 //Disable text mode parameter presentation.
OK
AT+CMGR=2 //Read the message whose <index> is 2.
+CMGR: "STO UNSENT","",
<This is a test from Quectel>
OK
AT+CSDH=1 //Enable text mode parameter presentation.
OK
AT+CMGR=2
+CMGR: "STO UNSENT","",,128,17,0,0,143,"+8613800551500",145,18
<This is a test from Quectel>
OK
8.16. AT+CSMP Set Text Mode Parameter
This Write Command selects values for additional parameters needed when SM is sent to the network or
placed in a storage when text mode is selected (AT+CMGF=1). It is also possible to set the validity
period starting from when a short message is received by SMSC (<vp> ranges from 0 to 255) or define
the absolute time of validity period termination (<vp> is a string).
AT+CSMP Set Text Mode Parameter
Test Command Response
AT+CSMP=? OK
Read Command Response
AT+CSMP? +CSMP: <fo>,<vp>,<pid>,<dcs>
OK
Write Command Response
AT+CSMP=<fo>[,<vp>[,<pid>[,<dcs>] OK
]] Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.005
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 194 / 282

5G Module Series
Parameter
<fo> Integer type. It depends on the command or result code: the first octet of
3GPP TS 23.040 SMS-DELIVER, SMS-SUBMIT (default value 17),
SMS-STATUS-REPORT or SMS-COMMAND. If a valid value has been entered once,
the parameter can be omitted.
<vp> Integer type or time-string type. Validity period. It depends on SMS-SUBMIT <fo>
setting: 3GPP TS 23.040 TP-Validity-Period (format see <dt>). Default value: 167.
<pid> Integer type. Protocol identifier. Default value: 0.
See 3GPP TS 23.040 TP-Protocol-Identifier.
<dcs> Integer type. Data coding scheme. It depends on the command or result code:
3GPP TS 23.038 SMS Data Coding Scheme (default value 0), or Cell Broadcast Data
Coding Scheme.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 195 / 282

5G Module Series
9
Packet Domain Commands
9.1. AT+CGATT Attach to or Detach from PS
This command attaches or detaches MT to/from the packet domain service. After the command has
been completed, MT remains in V.250 command state. If MT is already in the requested state, the
command will be ignored and the OK response returned. If the requested state cannot be achieved, an
ERROR or +CME ERROR response will be returned.
AT+CGATT Attach to or Detach from PS
Test Command Response
AT+CGATT=? +CGATT: (list of supported <state>s)
OK
Read Command Response
AT+CGATT? +CGATT: <state>
OK
Write Command Response
AT+CGATT=<state> OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 140 s, determined by the network.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<state> Integer type. PS attachment state.
0 Detached
1 Attached
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 196 / 282

5G Module Series
Other values are reserved and will result in an ERROR response to the Write Command.
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CGATT=1 //Attach to PS service.
OK
AT+CGATT=0 //Detach from PS service.
OK
AT+CGATT? //Query the current PS service attachment state.
+CGATT: 0
OK
9.2. AT+CGACT Activate or Deactivate PDP Context
This command activates or deactivates the specified PDP context(s). If a PDP context is already in the
requested state, the state for that context remains unchanged. Failure to achieve the requested state will
result in an ERROR or +CME ERROR. Extended error response is enabled by AT+CMEE.
If MT is not PS attached when the activation command is executed, MT will first attempt attachment and
then activate the specified context. In case of attachment failure, MT responds with ERROR or, if
extended error response is enabled, with the appropriate failure-to-attach error message.
For EPS, in case of an attempt to disconnect the last PDN connection, MT responds with ERROR, or, if
extended error response is enabled, it responds with +CME ERROR. The activation request for an EPS
bearer resource will be answered by the network by either an EPS dedicated bearer activation or an
EPS bearer modification request. The request must be accepted by MT before the PDP context can be
set to an established state.
For 5GS, the command is used to request or delete the specified QoS flow. The request for a specific
QoS flow will be answered by the network by either a PDU session establishment accept message or a
PDU session modification command message. The message must be accepted by the MT before the
QoS flow can be set to active state.
AT+CGACT Activate or Deactivate PDP Context
Test Command Response
AT+CGACT=? +CGACT: (list of supported <state>s)
OK
Read Command Response
AT+CGACT? +CGACT: <cid>,<state>
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 197 / 282

                                          5G Module Series
[…]

OK
| Write Command                      |     |     | Response  |
| ---------------------------------- | --- | --- | --------- |
| AT+CGACT=[<state>[,<cid1>[,<cid2>[ |     |     | OK        |
| ,…]]]]                             |     |     | Or        |
NO CARRIER

If there is any error:
ERROR
Or
+CME ERROR: <err>
| Maximum Response Time  |     |     | 150 s, determined by network.  |
| ---------------------- | --- | --- | ------------------------------ |
| Characteristics        |     |     | -                              |
| Reference              |     |     | 3GPP TS 27.007                 |
Parameter
| <state>   Integer type. PDP context activation status.  |                |     |     |
| ------------------------------------------------------- | -------------- | --- | --- |
|       0                                                 |   Deactivated  |     |     |
|       1                                                 |   Activated    |     |     |
      Other values are reserved and will result in an ERROR response to the Write Command.
<cid>    Integer type. Particular PDP context definition (see AT+CGDCONT).
| <err>    Error code. For more details, see Chapter 13.5.  |     |     |     |
| --------------------------------------------------------- | --- | --- | --- |
Example
| AT+CGDCONT=4,"IP","UNINET"  |     |     | //Define a PDP context.  |
| --------------------------- | --- | --- | ------------------------ |
OK
| AT+CGACT=1,4  |       |       | //PDP activated.   |
| ------------- | ----- | ----- | ------------------ |
OK
AT+CGACT?                             //Query the current PDP context state.
+CGACT: 1,1
+CGACT: 2,0
+CGACT: 3,0
+CGACT: 4,1
…

OK
| AT+CGACT=0,4  |       |       | //PDP deactivated.  |
| ------------- | ----- | ----- | ------------------- |
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                198 / 282

5G Module Series
9.3. AT+CGDATA Enter Data State
This Write Command causes MT to perform the necessary action to establish communication between
TE and the network using one or more packet domain PDP type(s). This may include performing a PS
attachment and one or more PDP context activation(s). Any command following AT+CGDATA in the AT
command line will not be processed by MT.
If the <L2P> value is unacceptable to MT, MT returns an ERROR or +CME ERROR. Otherwise, MT
issues the intermediate result code CONNECT and enters V.250 online data state. After data transfer is
completed and the layer 2 protocol termination procedure has been completed successfully, the V.250
command state is re-entered and MT returns the final result code OK.
AT+CGDATA Enter Data State
Test Command Response
AT+CGDATA=? +CGDATA: (list of supported <L2P>s)
OK
Write Command Response
AT+CGDATA=<L2P>,<cid> CONNECT
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<L2P> String type. Layer 2 protocol to be used between TE and MT:
"PPP" Point to Point protocol for PDP such as IP
Other values are not supported and will result in an ERROR response to the Write
Command.
<cid> Integer type. Particular PDP context definition (see AT+CGDCONT).
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 199 / 282

                                          5G Module Series
9.4.  AT+CGPADDR  Show PDP Addresses

This command returns a list of PDP addresses for the specified context identifiers. If no <cid> is
specified, the addresses for all defined contexts are returned.
AT+CGPADDR  Show PDP Address
| Test Command  |     |     | Response                            |
| ------------- | --- | --- | ----------------------------------- |
| AT+CGPADDR=?  |     |     | +CGPADDR: (list of defined <cid>s)  |

OK
| Execution/Write Command  |     |     | Response  |
| ------------------------ | --- | --- | --------- |
AT+CGPADDR=[<cid1>[,<cid2>[,…]]]  +CGPADDR: <cid>[,<PDP_addr_1>[,<PDP_addr_2>]]
[…]

OK

If there is any error:
ERROR
Or
+CME ERROR: <err>
| Maximum Response Time  |     |     | 300 ms          |
| ---------------------- | --- | --- | --------------- |
| Characteristics        |     |     | -               |
| Reference              |     |     | 3GPP TS 27.007  |
Parameter
<cid>    Integer type. Particular PDP context definition (see AT+CGDCONT).
<PDP_addr>   String type. MT in the address space applicable to PDP. The address may be static
        or dynamic. For a static address, it is the one set by AT+CGDCONT when the context
        is defined. For a dynamic address, it is the one assigned during the last PDP context
        activation that used the context definition referred to by <cid>. <PDP_addr> is
|            | omitted if no address is available.               |     |     |
| ---------- | ------------------------------------------------- | --- | --- |
| <err>      | Error code. For more details, see Chapter 13.5.   |     |     |
Example
| AT+CGDCONT=1,"IP","UNINET"  |     |     | //Define a PDP context.  |
| --------------------------- | --- | --- | ------------------------ |
OK
| AT+CGACT=1,1  |       |       | //PDP activated.  |
| ------------- | ----- | ----- | ----------------- |
OK
| AT+CGPADDR=1   |     |       | //Show the PDP address.  |
| -------------- | --- | ----- | ------------------------ |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                200 / 282

5G Module Series
+CGPADDR: 1,"10.76.51.180"
OK
9.5. AT+CGEREP Report Packet Domain Event
This command enables/disables sending of URC +CGEV from MT to TE in case of certain event
occurring in the packet domain MT or the network. <mode> controls the processing of URC specified
within this command. <bfr> controls the effect on buffered code when <mode> 1 or 2 is specified.
AT+CGEREP Report Packet Domain Event
Test Command Response
AT+CGEREP=? +CGEREP: (list of supported <mode>s),(list of supported
<bfr>s)
OK
Read Command Response
AT+CGEREP? +CGEREP: <mode>,<bfr>
OK
If there is any error:
ERROR
Write Command Response
AT+CGEREP=[<mode>[,<bfr>]] OK
Or
ERROR
Execution Command Response
AT+CGEREP OK
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<mode> Integer type.
0 Buffer URC in MT. If MT result code buffer is full, the oldest ones can be discarded.
No codes are forwarded to TE.
1 Discard URC when MT-TE link is reserved (e.g. in on-line data mode); otherwise
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 201 / 282

5G Module Series
forward directly to TE.
2 Buffer URC in MT when MT-TE link is reserved (e.g. in on-line data mode) and flush
URC to TE when MT-TE link becomes available; otherwise forward directly to TE.
<bfr> Integer type.
0 MT buffer of URC defined within this command is cleaned when <mode> 1 or 2
is specified.
1 MT buffer of URC defined within this command is flushed to TE when <mode> 1 or 2
is specified (OK should be given before flushing).
NOTE
The URCs and the corresponding events are defined as follows:
1. +CGEV: REJECT <PDP_type>,<PDP_addr>: A network request for PDP context activation occurs
when MT is unable to report it to TE with URC +CRING and is automatically rejected. This event is
not applicable for EPS and 5GS.
2. +CGEV: NW REACT <PDP_type>,<PDP_addr>[,<cid>]: The network has requested a context
reactivation. <cid> used to reactivate the context is provided if known to MT This event is not
applicable for EPS.
3. +CGEV: NW DEACT <PDP_type>,<PDP_addr>[,<cid>]: The network has forced a context
deactivation. <cid> used to activate the context is provided if known to MT.
4. +CGEV: ME DEACT <PDP_type>,<PDP_addr>[,<cid>]: The mobile equipment has forced a
context deactivation. <cid> used to activate the context is provided if known to the MT.
5. +CGEV: NW DETACH: The network has forced a packet domain detach. This implies that all active
contexts have been deactivated. These contexts are not reported separately.
6. +CGEV: ME DETACH: The mobile equipment has forced a packet domain detach. This implies that
all active contexts have been deactivated. These contexts are not reported separately.
7. +CGEV: NW CLASS<class>: The network has forced a change of MS class. The highest available
class is reported (see AT+CGCLASS in 3GPP 27.007 subclause 10.1.7).
8. +CGEV: ME CLASS<class>: The mobile equipment has forced a change of MS class. The highest
available class is reported (see AT+CGCLASS in 3GPP 27.007 subclause 10.1.7).
9. +CGEV: PDN ACT<cid>: activate a context. The context represents a PDN connection in LTE or a
primary PDP context in GSM/UMTS.
10. +CGEV: PDN DEACT<cid>: deactivate a context. The context represents a PDN connection in LTE
or a primary PDP context in GSM/UMTS.
Parameter
<PDP_type> String type. Packet data protocol type.
"IP" IPv4
"PPP" PPP
"IPV6" IPv6
"IPV4V6" IPv4v6
<PDP_addr> String type. MT in the address space applicable to PDP. If the value is null or
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 202 / 282

5G Module Series
omitted, then a value may be provided by TE during PDP.
<cid> Integer type. PDP context identifier. The parameter is local to the TE-MT interface
and is used in other PDP context-related commands. The range of permitted values
(minimum value=1) are returned by the test form of AT+CGDCONT.
<class> String type. GPRS mobile class.
A Class A (highest)
B Class B
C Class C in GPRS and circuit switched alternate mode
CG Class C in GPRS only mode
CC Class C in circuit switched only mode (lowest)
Example
AT+CGEREP=? //Test command.
+CGEREP: (0-2),(0,1)
OK
AT+CGEREP? //Query the current configuration.
+CGEREP: 0,0
OK
AT+CGEREP=2,1 //Report packet domain event.
OK
AT+CGACT=1,2 //A context activated.
OK
+CGEV: PDN ACT2
AT+CGACT=0,2 //A context deactivated.
OK
+CGEV: PDN DEACT2
9.6. AT+CGSMS Select Service for MO SMS Messages
This command specifies the service or service preference that MT will use to send MO (mobile originated)
SMS messages.
AT+CGSMS Select Service for MO SMS Messages
Test Command Response
AT+CGSMS=? +CGSMS: (list of currently available <service>s)
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 203 / 282

5G Module Series
OK
Read Command Response
AT+CGSMS? +CGSMS: <service>
OK
Write Command Response
AT+CGSMS=<service> OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<service> Integer type. Service or service preference to be used.
0 Packet domain
1 Circuit switched
2 Packet domain preferred (use circuit switched if GPRS not available)
3 Circuit switch preferred (use packet domain if circuit switched not available)
<err> Error code. For more details, see Chapter 13.5.
NOTE
Executing AT+CGSMS=<service> writes data to NVM. Please proceed with caution.
9.7. AT+QGDNRCNT Packet Data Counter (5G Supported)
This command queries the data traffic information sent and received by MT. Compared with
AT+QGDCNT, this AT command further supports the packet data counter in 5G network.
AT+QGDNRCNT Packet Data Counter (5G Supported)
Test Command Response
AT+QGDNRCNT=? +QGDNRCNT: (list of supported <op>s)
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 204 / 282

5G Module Series
OK
Read Command Response
AT+QGDNRCNT? +QGDNRCNT: <bytes_sent>,<bytes_recv>
OK
Write Command Response
AT+QGDNRCNT=<op> OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference -
Parameter
<op> Integer type. Operation about packet data counter.
0 Reset the packet data counter
1 Save the result of the packet data counter to NVM
<bytes_sent> Integer type. Byte(s) of sent data traffic.
<bytes_recv> Integer type. Byte(s) of received data traffic.
<err> Error code. For more details, see Chapter 13.5.
NOTE
1. Once MT is powered on, it retrieves the values for <bytes_recv> and <bytes_sent> from the
packet data counter in NVM. The default value in NVM is 0.
2. AT+QGDNRCNT=1 can write the data traffic to NVM and it should not be executed frequently,
otherwise the service life of the module flash will be shortened. If you need to write the data traffic to
NVM, it is recommended that the interval between such operations is more than 60 seconds.
3. Executing AT+QGDNRCNT=0 or AT+QGDNRCNT=1 writes data to NVM. Please proceed with
caution.
Example
AT+QGDNRCNT=? //Test command.
+QGDNRCNT: (0,1)
OK
AT+QGDNRCNT? //Query the current sent and received data.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 205 / 282

5G Module Series
+QGDNRCNT: 3832,4618
OK
AT+QGDNRCNT=1 //Write the data traffic to NVM.
OK
AT+QGDNRCNT=0 //Reset the packet data counter.
OK
9.8. AT+QAUGDCNT Auto Save Packet Data Counter
This command allows AT+QGDNRCNT to save the result to NVM automatically.
AT+QAUGDCNT Auto Save Packet Data Counter
Test Command Response
AT+QAUGDCNT=? +QAUGDCNT: (list of supported <value>s)
OK
Read Command Response
AT+QAUGDCNT? +QAUGDCNT: <value>
OK
Write Command Response
AT+QAUGDCNT=<value> OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference -
Parameter
<value> Integer type. Time-interval for AT+QGDNRCNT to save the result to NVM automatically.
Range: 0, 30–65535. Default value: 0. Unit: second. If it is set to 0, auto-save feature is
disabled.
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 206 / 282

5G Module Series
NOTE
After this command is enabled, the module writes data to flash based on the time interval, which may
reduce the service life of flash.
Example
AT+QAUGDCNT=? //Test command.
+QAUGDCNT: (0,30-65535)
OK
AT+QAUGDCNT=35 //Set <value> to 35.
OK
AT+QAUGDCNT? //Query the interval of auto-save.
+QAUGDCNT: 35
OK
9.9. AT+QNETDEVSTATUS Query RmNet Device Status
This command queries RmNet device status.
AT+QNETDEVSTATUS Query RmNet Device Status
Test Command Response
AT+QNETDEVSTATUS=? +QNETDEVSTATUS: (list of supported <on_off>s)
OK
Or
ERROR
Read Command Response
AT+QNETDEVSTATUS? If an RmNet call is in progress, <state>, <IP_type> and
<profile_num> are included:
+QNETDEVSTATUS: <on_off>[,<state>,<IP_type>,<profil
e_num>]
[...]
OK
Write Command Response
AT+QNETDEVSTATUS=<on_off> OK
Or
ERROR
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 207 / 282

5G Module Series
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is not saved.
Parameter
<on_off> Integer type. Enable/Disable URC reporting RmNet device status.
0 Disable URC reporting RmNet device status
1 Enable URC reporting RmNet device status
<state> Integer type. RmNet call status.
0 RmNet call is disconnected
1 RmNet call is connected
<IP_type> Integer type. IP type.
4 IPv4
6 IPv6
<profile_num> Integer type. Profile number. Range: 1–42.
Example
AT+QNETDEVSTATUS=?
+QNETDEVSTATUS:(0,1)
OK
AT+QNETDEVSTATUS?
+QNETDEVSTATUS: 1
OK
AT+QNETDEVSTATUS?
+QNETDEVSTATUS: 1,1,4,1
+QNETDEVSTATUS: 1,1,6,1
OK
+QNETDEVSTATUS: 1,0,4,1
+QNETDEVSTATUS: 1,0,6,1
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 208 / 282

5G Module Series
10
Supplementary Service
Commands
10.1. AT+CCFC Call Forwarding Number and Conditions Control
This command allows control of the call forwarding supplementary service according to 3GPP TS 22.082.
Registration, erasure, activation, deactivation and status query are supported.
AT+CCFC Call Forwarding Number and Conditions Control
Test Command Response
AT+CCFC=? +CCFC: (list of supported <reads>s)
OK
Write Command Response
AT+CCFC=<reads>,<mode>[,<numbe If <mode> is not equal to 2 and the command is executed
r>[,<type>[,<class>[,<subaddr>[,<sat successfully:
ype>[,<time>]]]]]] OK
If <mode>=2 and the command is executed successfully
(only in connection with <reads>=(0–3)):
For registered call forwarding number:
+CCFC: <status>,<class1>[,<number>,<type>[,<subadd
r>,<satype>[,<time>]]]
[...]
OK
If no call forwarding number is registered (and therefore all
classes are inactive):
+CCFC: <status>,<class>
OK
If there is any error:
ERROR
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 209 / 282

5G Module Series
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<reads> Integer type. Call forwarding condition.
0 Unconditional
1 Mobile busy
2 No reply
3 Not reachable
4 All call forwarding (see 3GPP TS 22.030)
5 All conditional call forwarding (see 3GPP TS 22.030)
<mode> Integer type. Operation type.
0 Disable
1 Enable
2 Query status
3 Register
4 Erasure
<number> String type. Phone number of forwarding address in format specified by <type>.
<type> Integer type. Address type. The default value is 145 when the dialing string includes
international access code character "+"; otherwise 129.
<subaddr> String type. Sub-address in the format specified by <satype>.
<satype> Integer type. Sub-address type.
<class> Integer type. Each represents a class of information.
1 Voice (telephony)
2 Data (various bearer services included, with exceptions in cases where TA
does not support values 16, 32, 64 and 128 with <mode>=2)
4 Fax (facsimile services)
7 Voice, data and fax
8 Short message service
16 Data circuit synchronization
32 Data circuit asynchronization
64 Dedicated packet access
128 Dedicated PAD access
<time> Integer type. Time in seconds to wait before a call is forwarded when "no reply", "all call
forwarding" or "all conditional call forwarding" is enabled or queried. Range: 1–30. Default
value: 20
<status> Integer type.
0 Not active
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 210 / 282

5G Module Series
1 Active
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CCFC=0,3,"15021012496" //Register the destination number for unconditional call
OK forwarding (CFU).
AT+CCFC=0,2 //Query the status of CFU without specifying <class>.
+CCFC: 1,1,"+8615021012496",145,,,
OK
AT+CCFC=0,4 //Erase the registered CFU destination number.
OK
AT+CCFC=0,2 //Query the status and there is no destination number.
+CCFC: 0,255
OK
10.2. AT+CCWA Call Waiting Control
This command allows control of the call waiting supplementary service according to 3GPP TS 22.083.
Activation, deactivation and status query are supported.
AT+CCWA Call Waiting Control
Test Command Response
AT+CCWA=? +CCWA: (list of supported <n>s)
OK
Read Command Response
AT+CCWA? +CCWA: <n>
OK
Write Command Response
AT+CCWA=[<n>[,<mode>[,<class>]]] If <mode> is not equal to 2 and the command is executed
successfully:
OK
If <mode>=2 and the command is executed successfully:
+CCWA: <status>,<class1>
[<CR><LF>+CCWA: <status>,<class2>
[...]]
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 211 / 282

                                          5G Module Series
OK

If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time  300 ms
Characteristics  -
Reference  3GPP TS 27.007
Parameter
<n>       Integer type. Enable or disable URC presentation. When call waiting is enabled and
        active on MT, and a terminating call occurs during an ongoing call, an URC is returned
|       |   in the following format:  |     |
| ----- | --------------------------- | --- |
        +CCWA: <number>,<type>,<class>[,<alpha>][,<CLI_validity>[,<subaddr>,<saty
|       |   pe>[,<priority>]]]  |           |
| ----- | --------------------- | --------- |
|       |   0                   | Disable   |
|       |   1                   | Enable    |
<mode>     Integer type. When <mode> is omitted, the network is not interrogated.
|       |   0    | Disable       |
| ----- | ------ | ------------- |
|       |   1    | Enable        |
|       |   2    | Query status  |
<class>      Integer type. Each represents a class of information.
|       |   1    | Voice (telephony)  |
| ----- | ------ | ------------------ |
2  Data (various bearer services included, with exceptions in cases where TA
does not support values 16, 32, 64 and 128 with <mode>=2)
|       |   4     | Fax (facsimile services)       |
| ----- | ------- | ------------------------------ |
|       |   7     | Voice, data and fax            |
|       |   8     | Short message service          |
|       |   16    | Data circuit synchronization   |
|       |   32    | Data circuit asynchronization  |
|       |   64    | Dedicated packet access        |
|       |   128   | Dedicated PAD access           |
<status>    Integer type. It indicates whether the status of the command is enabled or not.
|        |   0    | Disable   |
| ------ | ------ | --------- |
|        |   1    | Enable    |
<number>    String type. Phone number of calling address in format specified by <type>.
| <type>    |   Integer type. Address type in octet.  |                                          |
| --------- | --------------------------------------- | ---------------------------------------- |
|           |   128                                   | Type specified by the network            |
|           |   129                                   | Unknown type (ISDN format number)        |
|           |   145                                   | International number type (ISDN format)  |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                212 / 282

5G Module Series
<alpha> Optional string type. Alphanumeric representation of <number> corresponding to the
entry found in the phonebook.
<CLI_validity> Integer type. Reason why <number> does not contain a calling party BCD number
(see 3GPP TS 24.008 subclause 10.5.4.30).
0 CLI valid
1 CLI has been withheld by the originator
(see 3GPP TS 24.008table 10.5.135a/3GPP TS 24.008 code "Reject by user")
2 CLI is not available due to interworking problems or limitations of originating
network (see 3GPP TS 24.008 table 10.5.135a/3GPP TS 24.008 code
"Interaction with other service")
3 CLI is not available due to calling party being of type payphone
(see 3GPP TS 24.008 table 10.5.135a/3GPP TS 24.008 code "Coin
line/payphone")
4 CLI is not available due to other reasons (see 3GPP TS 24.008
table 10.5.135a/3GPP TS 24.008 code "Unavailable")
If CLI is unavailable (<CLI_validity>=2, 3 or 4), <number> is an empty string ("") and
<type> will not be significant. Nevertheless, TA may return the recommended value
128 for <type> (TON/NPI unknown in accordance with 3GPP TS 24.008
subclause 10.5.4.7).
When CLI has been withheld by the originator, (<CLI_validity>=1) and CLIP is
provisioned with the "override category" option (see 3GPP TS 22.081 and
3GPP TS 23.081), both <number> and <type> are provided. Otherwise, TA returns
the same settings for <number> and <type> as if CLI is unavailable.
<subaddr> String type. Sub-address of format specified by <satype>.
<satype> Integer type. Sub-address in octet (see 3GPP TS 24.008 subclause 10.5.4.8).
<priority> Optional digit type. eMLPP priority level of the incoming call. Priority level value is as
defined in eMLPP specification 3GPP TS 22.067.
<err> Error code. For more details, see Chapter 13.5.
NOTE
1. <status>=0 should be returned only if the service is not active for <class>, i.e., +CCWA: 0,7 will be
returned in this case.
2. When <mode>=2, all active call waiting classes will be reported. In this mode the command is
aborted by pressing any key.
3. Executing AT+CCWA=<n>,<mode>,<class> writes data to NVM. Please proceed with caution.
Example
AT+CCWA=1,1 //Enable URC presentation.
OK
ATD10086; //Establish a call.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 213 / 282

5G Module Series
+CCWA: "02154450293",129,1 //Waiting call.
10.3. AT+CHLD Call-Related Supplementary Services
This command allows the control of the following call-related services:
⚫ Temporary disconnection of a call from MT, while retaining the connection with the network;
⚫ Multiparty conversation (conference calls);
⚫ The served subscriber with two calls (one on hold and the other either active or alerting) can
connect other parties and release their own connection.
The call can be put on hold, recovered, released and added to a conversation, and transferred similarly
as defined in 3GPP TS 22.030.
This is based on the GSM/UMTS supplementary services:
⚫ HOLD (Call Hold; see 3GPP TS 22.083 clause 2);
⚫ MPTY (MultiParty; see 3GPP TS 22.084);
⚫ ECT (Explicit Call Transfer; see 3GPP TS 22.091).
The interaction of this command with other commands based on other GSM/UMTS supplementary
services is described in the GSM/UMTS standards. Call Hold, MultiParty and Explicit Call Transfer are
only applicable to teleservice 11.
AT+CHLD Call-Related Supplementary Services
Test Command Response
AT+CHLD=? +CHLD: (list of supported <n>s)
OK
Write Command Response
AT+CHLD=[<n>] OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 214 / 282

                                          5G Module Series
Parameter
| <n>     | Integer type.   |     |     |
| ------- | --------------- | --- | --- |
      0    Terminate all held calls or UDUB (User Determined User Busy) for a waiting call.
          If a call is waiting, terminate the waiting call. Otherwise, terminate all held calls (if
|       |     any)  |     |     |
| ----- | --------- | --- | --- |
      1    Terminate all active calls (if any) and accept the other call (waiting or held call).
|       | 1X    Terminate the specific call number X (X = 1–7)  |     |     |
| ----- | ----------------------------------------------------- | --- | --- |
      2    Place all active calls on hold (if any) and accept the other call (waiting call or held
|       |     call) as the active call.   |     |     |
| ----- | ------------------------------- | --- | --- |
      2X    Place all active calls except call X (X = 1–7) on hold
|       | 3    Add a held call to active calls  |     |     |
| ----- | ------------------------------------- | --- | --- |
      4      Connect the two calls and disconnect the subscriber from both calls (ECT)
| <err>      | Error code. For more details, see Chapter 13.5.  |     |     |
| ---------- | ------------------------------------------------ | --- | --- |
Example
| ATD10086;  |       |       |   //Establish a call.  |
| ---------- | ----- | ----- | ---------------------- |
OK

+CCWA: "02154450293",129,1      //Indication of a waiting call.
AT+CHLD=2               //Place the active call on hold and accept the waiting call as
|       |       |       |    the active call.  |
| ----- | ----- | ----- | -------------------- |
OK
AT+CLCC
| +CLCC: 1,0,1,0,0,"10086",129   |     |     |   //The first call is on hold.  |
| ------------------------------ | --- | --- | ------------------------------- |
+CLCC: 2,1,0,0,0,"02154450293",129    //The second call is active.

OK
AT+CHLD=21              //Place the active call except call X = 1 on hold.
OK
AT+CLCC
| +CLCC: 1,0,0,0,0,"10086",129   |     |     |   //The first call is active.  |
| ------------------------------ | --- | --- | ------------------------------ |
+CLCC: 2,1,1,0,1,"02154450293",129    //The second call is on hold.

OK
AT+CHLD=3                               //Add a held call to active calls to set up a conference (multiparty) call.
OK
AT+CLCC
| +CLCC: 1,0,0,0,1,"10086",129   |     |     |   //The first call is active.  |
| ------------------------------ | --- | --- | ------------------------------ |
+CLCC: 2,1,0,0,1,"02154450293",129    //The second call is active.

OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                215 / 282

5G Module Series
10.4. AT+CLIP Present Calling Line Identification
This command refers to the GSM/UMTS supplementary service CLIP (Calling Line Identification
Presentation) that enables a called subscriber to get the calling line identity (CLI) of the calling party
when receiving a mobile terminated call. It has no effect on the execution of the supplementary service
CLIP in the network.
AT+CLIP Present Calling Line Identification
Test Command Response
AT+CLIP=? +CLIP: (list of supported <n>s)
OK
Read Command Response
AT+CLIP? +CLIP: <n>,<m>
OK
Write Command Response
AT+CLIP=[<n>] OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 15 s, determined by network.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<n> Integer type. Enable or disable reporting of URC presentation status to TE.
When the CLIP presentation at TE is enabled (and permitted by the calling
subscriber), an URC is returned after every RING (or +CRING: <type>) at a mobile
terminating call:
+CLIP: <number>,<type>,[subaddr],[satype],[<alpha>],<CLI_validity>
0 Disable URC
1 Enable URC
<m> Integer type. Subscriber CLIP service status in the network.
0 CLIP not provisioned
1 CLIP provisioned
2 Unknown (e.g., no network, etc.)
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 216 / 282

5G Module Series
<number> String type. Phone number calling address in format specified by <type>.
<subaddr> String type. Sub-address of format specified by <satype>.
<satype> Integer type. Sub-address type in octet (see 3GPP TS 24.008 subclause 10.5.4.8)
<type> Integer type. Address type in octet.
129 Unknown type (ISDN format)
145 International number type (ISDN format)
161 National number
<alpha> String type alphanumeric representation of <number> corresponding to the entry
found in the phonebook.
<CLI_validity> Integer type. Reason why <number> does not contain a calling party BCD number.
0 CLI valid
1 CLI has been withheld by the originator
2 CLI is not available due to interworking problems or limitations of originating
network
<err> Error code. For more details, see Chapter 13.5.
Example
AT+CPBW=1,"02151082965",129,"QUECTEL"
OK
AT+CLIP=1
OK
RING
+CLIP: "02151082965",129,,,"QUECTEL",0
10.5. AT+CLIR Restrict Calling Line Identification
This command refers to the CLIR supplementary service (Calling Line Identification Restriction)
according to 3GPP TS 22.081 and the OIR supplementary service (Originating Identification Restriction)
according to 3GPP TS 24.607 that allows a calling subscriber to enable or disable the presentation of the
calling line identity (CLI) to the called party when originating a call.
The Write Command overrides the CLIR subscription (default is restricted or allowed) when temporary
mode is provisioned as a default adjustment for all following outgoing calls. This adjustment can be
revoked by using the opposite command.
AT+CLIR Restrict Calling Line Identification
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 217 / 282

5G Module Series
Test Command Response
AT+CLIR=? +CLIR: (list of supported <n>s)
OK
Read Command Response
AT+CLIR? +CLIR: <n>,<m>
OK
Write Command Response
AT+CLIR=<n> OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 15 s, determined by network.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<n> Integer type. Outgoing call adjustment.
0 Presentation indicator is used according to the subscription of the CLIR service
1 CLIR invocation
2 CLIR suppression
<m> Integer type. Subscriber CLIR service status in the network.
0 CLIR not provisioned
1 CLIR provisioned in permanent mode
2 Unknown (e.g., no network, etc.)
3 CLIR temporary mode presentation restricted
4 CLIR temporary mode presentation allowed
<err> Error code. For more details, see Chapter 13.5.
10.6. AT+COLP Present Connected Line Identification
This command enables/disables a calling subscriber to get the connected line identity (COL) of the
called party after setting up a mobile originated call, referring to the GSM/UMTS supplementary service
COLP (Connected Line Identification Presentation). MT enables or disables COL presentation at TE for a
mobile originated a call. It has no effect on the execution of the supplementary service COLR in the
network.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 218 / 282

5G Module Series
AT+COLP Present Connected Line Identification
Test Command Response
AT+COLP=? +COLP: (list of supported <n>s)
OK
Read Command Response
AT+COLP? +COLP: <n>,<m>
OK
Write Command Response
AT+COLP=[<n>] OK
Or
ERROR
Maximum Response Time 15 s, determined by network.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<n> Integer type. Set/present the result code presentation status in MT.
0 Disable
1 Enable. When enabled (and permitted by the calling subscriber), an
intermediate result code is returned before any +CR or V.25ter responses:
+COLP: <number>,<type>,[<subaddr>],[<satype>],[<alpha>]
<m> Integer type. Subscriber COLP service status in the network.
0 COLP not provisioned
1 COLP provisioned
2 Unknown (e.g., no network, etc.)
<number> String type. Phone number; calling address in format specified by <type>.
<type> Integer type. Address type in octet.
129 Unknown type (ISDN format number)
145 International number type (ISDN format)
<subaddr> String type. Sub-address of format specified by <satype>.
<satype> Integer type. Sub-address type in octet (see 3GPP TS 24.008 subclause 10.5.4.8).
<alpha> Optional string. Alphanumeric representation of <number> corresponding to the entry
found in the phonebook.
Example
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 219 / 282

5G Module Series
AT+CPBW=1,"02151082965",129,"QUECTEL"
OK
AT+COLP=1
OK
ATD02151082965;
+COLP: "02151082965",129,,,"QUECTEL"
OK
10.7. AT+CSSN Supplementary Service Notification
This command enables or disables the presentation of notification result codes from TA to TE.
AT+CSSN Supplementary Service Notification
Test Command Response
AT+CSSN=? +CSSN: (list of supported <n>s),(list of supported <m>s)
OK
Read Command Response
AT+CSSN? +CSSN: <n>,<m>
OK
Write Command Response
AT+CSSN=<n>[,<m>] OK
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<n> Integer type. Enable or disable the +CSSI intermediate result code presentation status to
TE. When <n>=1 and a supplementary service notification is received after a mobile
originated call is set up, the +CSSI: <code1> intermediate result code is sent to TE before
any other MO call setup result code.
0 Disable
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 220 / 282

5G Module Series
1 Enable
<m> Integer type. Enable or disable the +CSSU URC presentation status to TE. When <m>=1
and a supplementary service notification is received during a mobile terminated call setup
or during a call, the +CSSU: <code2> URC is sent to TE.
0 Disable
1 Enable
<code1> Integer type. It is specified by manufacturer and supports the following codes:
0 Unconditional call forwarding is active
1 Some of the conditional call forwarding options are active
2 Call has been forwarded
3 Call is waiting
5 Outgoing call is barred
<code2> Integer type. It is specified by manufacturer and supports the following codes:
0 Incoming call is a forwarded call
2 Call has been put on hold (during a voice call)
3 Call has been retrieved (during a voice call)
5 Held call terminated by another party
10 Additional incoming call forwarded
<err> Error code. For more details, see Chapter 13.5.
10.8. AT+CUSD Unstructured Supplementary Service Data
This command allows control of the USSD (Unstructured Supplementary Service Data) according to
3GPP TS 22.090. Both network and mobile initiated operations are supported.
<mode> disables/enables the presentation of an URC. The value <mode>=2 cancels an ongoing USSD
session. For USSD response from the network, or network initiated operation, the format is: +CUSD:
<status>[,<rspstr>,[<dcs>]].
When <reqstr> is given, a mobile-initiated USSD string or a response USSD string to a network-initiated
operation is sent to the network. The response USSD string from the network is returned in a subsequent
+CUSD URC.
AT+CUSD Unstructured Supplementary Service Data
Test Command Response
AT+CUSD=? +CUSD: (list of supported <mode>s)
OK
Read Command Response
AT+CUSD? +CUSD: <mode>
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 221 / 282

5G Module Series
Write Command Response
AT+CUSD=[<mode>[,<reqstr>[,<dcs> OK
]]]
If there is any error:
ERROR
Or
+CME ERROR: <err>
Maximum Response Time 120 s, determined by the network.
Characteristics -
Reference 3GPP TS 27.007
Parameter
<mode> Integer type. Enable or disable the result code presentation status to TE.
0 Disable
1 Enable
2 Cancel session (not applicable to the Read Command response)
<reqstr> String type. USSD to be sent to the network. If this parameter is omitted, the network is
not queried.
<rspstr> String type. USSD received from the network
<dcs> Integer type. See 3GPP TS 23.038 Cell Broadcast Data Coding Scheme (default value
15).
<status> Integer type. USSD response from the network or the network-initiated operation
0 No further user action required (network initiated USSD Notify, or no further
information needed after mobile initiated operation)
1 Further user action required (network initiated USSD Request, or further
information needed after mobile initiated operation)
2 USSD terminated by network
3 Another local client has responded
4 Operation not supported
5 Network time out
<err> Error code. For more details, see Chapter 13.5.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 222 / 282

5G Module Series
11
Hardware-Related Commands
11.1. AT+QPOWD Power off
This command powers off MT. Once the command is executed successfully, UE returns OK immediately
and deactivates the network. After the deactivation is completed, UE outputs POWERED DOWN and
enters power-off state. The maximum time for unregistering network is 60 seconds. To avoid data loss,
the power supply for the module cannot be disconnected before POWERED DOWN is outputted.
AT+QPOWD Power off
Test Command Response
AT+QPOWD=? +QPOWD: (list of supported <n>s)
OK
Write Command Response
AT+QPOWD=[<n>] OK
POWERED DOWN
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference
Parameter
<n> Integer type.
0 Immediate power-down
1 Normal power-down
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 223 / 282

                                          5G Module Series
11.2. AT+CCLK  Clock

This command sets and queries the real time clock (RTC) of MT. The current setting is retained until MT
is totally disconnected from the power supply.
AT+CCLK  Clock
| Test Command  |     |     | Response       |
| ------------- | --- | --- | -------------- |
| AT+CCLK=?     |     |     | OK             |
| Read Command  |     |     | Response       |
| AT+CCLK?      |     |     | +CCLK: <time>  |

OK
| Write Command   |     |     | Response  |
| --------------- | --- | --- | --------- |
| AT+CCLK=<time>  |     |     | OK        |

If there is any error:
ERROR
Or
+CME ERROR: <err>
| Maximum Response Time  |     |     | 300 ms  |
| ---------------------- | --- | --- | ------- |
The command takes effect immediately.
Characteristics
The configuration is not saved.
| Reference  |     |     | 3GPP TS 27.007  |
| ---------- | --- | --- | --------------- |
Parameter
<time>  String type. Format: "yy/MM/dd,hh:mm:ss±zz", where characters indicate year (two last
digits),  month,  day,  hour,  minutes,  seconds  and  time  zone  (indicating  the  difference,
expressed in quarter(s) of an hour, between the local time and GMT; range: -48 to +56).
E.g. May 6th, 1994, 22:10:00 GMT+2 hours equals "94/05/06,22:10:00+08".
| <err>  | Error code. For more details, see Chapter 13.5.   |     |     |
| ------ | ------------------------------------------------- | --- | --- |
Example
| AT+CCLK?  |       |       |   //Query the local time.  |
| --------- | ----- | ----- | -------------------------- |
+CCLK: "08/01/04,00:19:43+00"

OK

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                224 / 282

5G Module Series
11.3. AT+CBC Battery Charge
This command returns battery charge status <bcs> and battery charge level <bcl> of MT.
AT+CBC Battery Charge
Test Command Response
AT+CBC=? +CBC: (list of supported <bcs>s),(list of supported
<bcl>s),<voltage>
OK
Execution Command Response
AT+CBC +CBC: <bcs>,<bcl>,<voltage>
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Reference 3GPP TS 27.007
Parameter
<bcs> Integer type. Battery charge status.
0 ME is not charging
1 ME is charging
2 Charging has been finished
<bcl> Integer type. Battery charge level in percent. Range: 0–100
<voltage> Battery voltage. Unit: mV.
<err> Error code. For more details, see Chapter 13.5.
11.4. AT+QADC Read ADC Value
This command reads the voltage value of ADC channel.
AT+QADC Read ADC Value
Test Command Response
AT+QADC=? +QADC: (list of supported <port>s)
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 225 / 282

5G Module Series
OK
Read Command Response
AT+QADC=<port> +QADC: <status>,<value>
OK
Maximum Response Time 300 ms
Characteristics -
Parameter
<port> Integer type. ADC channel number.
0 ADC channel 0
1 ADC channel 1
<status> Integer type. Whether the ADC value has been read successfully.
0 Failed
1 Successful
<value> Integer type. Voltage of specified ADC channel. Unit: uV.
11.5. AT+QSCLK Set Sleep Mode
This command controls whether to enable MT to enter sleep mode. When sleep mode is enabled, MT
can enter sleep mode directly.
AT+QSCLK Set Sleep Mode
Test Command Response
AT+QSCLK=? +QSCLK: (list of supported <n>s),(list of supported
<saved>s)
OK
Read Command Response
AT+QSCLK? +QSCLK: <n>,<saved>
OK
Write Command Response
AT+QSCLK=<n>[,<saved>] OK
If there is any error:
ERROR
Maximum Response Time 300 ms
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 226 / 282

5G Module Series
Characteristics -
Reference -
Parameter
<n> Integer type. Enable or disable sleep mode.
0 Disable
1 Enable. It is controlled by DTR. DTR is pulled up by default.
<saved> Integer type. Whether to save the configuration into NVM.
0 Do not save
1 Save
NOTE
Executing AT+QSCLK=0,1 or AT+QSCLK=1,1 writes data to NVM. Please proceed with caution.
11.6. AT+QAGPIO Set Output Level of AP or PMU GPIO
This command sets the AP or PMU GPIO output level.
AT+QAGPIO Set Output Level of AP or PMU GPIO
Test Command Response
AT+QAGPIO=? +QAGPIO: <type>,<gpio_num>,(list of supported
<value>s)
OK
Write Command Response
AT+QAGPIO=<type>,<gpio_num>,<v OK
alue> Or
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Parameter
<type> Integer type. Set up AP or PMU.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 227 / 282

5G Module Series
0 AP
1 PMU
<gpio_num> Integer type. GPIO number. When <type> is 0, the range is 0–107. When <type> is
1, the range is 1–16.
<value> Integer type. GPIO output level.
0 Low level
1 High level
NOTE
PMU GPIO range: 1–16.
Example
AT+QAGPIO=? //Test Command
+QAGPIO: <type>,<gpio_num>,(0,1)
OK
AT+QAGPIO=0,105,1 //Set the AP gpio_105 output to high level.
OK
AT+QAGPIO=1,8,0 //Set the PMU gpio_8 output to low level.
OK
11.7. AT+QETH="eth_driver" Select PCIe Ethernet Controller Driver to be
Loaded
This command selects the PCIe Ethernet controller driver to be loaded when the module starts up.
AT+QETH="eth_driver" Select PCIe Ethernet Controller Driver to be Loaded
Write Command Response
AT+QETH="eth_driver"[,<driver If the optional parameters are omitted, query the current setting.
_name>,<status>] +QETH: "eth_driver",<driver_name>,<status>
OK
If the optional parameters are specified, select the PCIe Ethernet
controller driver to be loaded when the module starts up.
OK
If there is any error:
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 228 / 282

                                          5G Module Series
ERROR
| Maximum Response Time  |     |     | 300 ms  |
| ---------------------- | --- | --- | ------- |
The command takes effect after the module is rebooted.
Characteristics
The configurations are saved automatically.
Parameter
| <driver_name>  | String type. PCIe Ethernet controller driver name.  |                                                 |     |
| -------------- | --------------------------------------------------- | ----------------------------------------------- | --- |
|                | "r8125"                                             |   RTL8125 2.5G PCIe Ethernet controller driver  |     |
"r8168"    RTL8111/RTL8119/RTL8168 1G PCIe Ethernet controller driver
<status>  Integer  type.  Whether  to  load  the  Ethernet  controller  driver  specified  by
<driver_name> when the module starts up.
|     | 0   | Not load  |     |
| --- | --- | --------- | --- |
|     | 1   | Load      |     |
Example
AT+QETH="eth_driver","r8125",1     //Load RTL8125 2.5G PCIe Ethernet controller driver when
|       |       |       |   the module starts up.  |
| ----- | ----- | ----- | ------------------------ |
OK
AT+QETH="eth_driver","r8125",0      //Do not load RTL8125 2.5G PCIe Ethernet controller driver
|       |       |       |   when the module starts up.  |
| ----- | ----- | ----- | ----------------------------- |
OK
| AT+QETH="eth_driver"  |     |       |   //Query the current setting.  |
| --------------------- | --- | ----- | ------------------------------- |
+QETH: "eth_driver","r8125",0

OK

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                229 / 282

5G Module Series
12
QMAP-Related Commands
12.1. AT+QMAP Configure QMAP-Related Parameters
This command configures QMAP-related parameters.
AT+QMAP Configure QMAP-Related Parameters
Test Command Response
AT+QMAP? +QMAP: "WWAN",(list of supported <status>s),(list of
supported <profileID>s),(list of supported
<IP_family>s),<IP_address>
+QMAP: "DMZ",(list of supported <enable>s),(list of supported
<IP_family>s),<IP_address>
+QMAP: "GRE",(list of supported <enable>s),<IP_address>
+QMAP: "LAN",<IP_address>
+QMAP: "LANIP",<LAN_IP_start_address>,<LAN_IP_end_a
ddress>,<GW_IP_address>,(list of supported <effect>s)
+QMAP: "VLAN",(list of supported <VLAN_ID>s),(list of
supported <enable>s),(list of supported <VLAN_type>s)
+QMAP: "MPDN_rule",(list of supported <rule_num>s),(list of
supported <profileID>s),(list of supported <VLAN_ID>s),(list of
supported <IPPT_mode>s),(list of supported
<auto_connect>s),<ippt_info>
+QMAP: "IPPT_NAT",(list of supported <IPPT_NAT>s)
+QMAP: "connect",(list of supported <rule_num>s),(list of
supported <connect>s)
+QMAP: "auto_connect",(list of supported <rule_num>s),(list
of supported <auto_connect>s),(list of supported
<profileID>s)
+QMAP: "MPDN_status"
+QMAP: "SFE",(list of supported <status>s)
+QMAP: "domain",<domain_name>
+QMAP: "DHCPV4DNS",(list of supported <status>s)
+QMAP: "DHCPV6DNS",(list of supported <status>s)
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 230 / 282

5G Module Series
Maximum Response Time 300 ms
Characteristics -
12.2. AT+QMAP="WWAN" Query IP Address of Default QMAP Data Call
This command queries the status and IP address of the default QMAP data call.
AT+QMAP="WWAN" Query IP Address of Default QMAP Data Call
Write Command Response
AT+QMAP="WWAN" +QMAP: "WWAN",<status>,<profileID>,<IP_family>,<IP_address>
+QMAP: "WWAN",<status>,<profileID>,<IP_family>,<IP_address>
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<status> Integer type. Status of default QMAP data call.
0 Disconnected
1 Connected
<profileID> Integer type. Profile ID of default QMAP data call. Range: 1–16.
<IP_family> String type IP type.
"IPV4" IPv4
"IPV6" IPv6
<IP_address> String type. IP address of default QMAP data call.
If IPv4 network is not connected, the address is "0.0.0.0".
If IPv6 network is not connected, the address is "0:0:0:0:0:0:0:0".
Example
AT+QMAP="WWAN" //Query IP address of default QMAP data call
+QMAP: "WWAN",0,1,"IPV4","0.0.0.0"
+QMAP: "WWAN",0,1,"IPV6","0:0:0:0:0:0:0:0"
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 231 / 282

5G Module Series
12.3. AT+QMAP="DMZ" Query/Set DMZ of Default QMAP Data Call
This command queries or sets DMZ (Demilitarized Zone) of the default QMAP data call.
AT+QMAP="DMZ" Query/Set DMZ of Default QMAP Data Call
Write Command Response
AT+QMAP="DMZ"[,<enable>,<IP_f If the optional parameters are omitted, query the current setting:
amily>[,<IP_address>]] +QMAP: "DMZ",<enable>,<IP_family>[,<IP_address>]
+QMAP: "DMZ",<enable>,<IP_family>[,<IP_address>]
OK
If any of the optional parameters is specified, enable or disable
DMZ:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Parameter
<enable> Integer type. Enable or disable DMZ of default QMAP data call.
0 Disable
1 Enable
<IP_family> Integer type. IP type.
4 IPv4
6 IPv6
<IP_address> String type. Dotted decimal IPv4 or IPv6 address without double quotes. It is valid
only when <enable> is 1.
NOTE
1. After DMZ is enabled, to change the DMZ address, disable DMZ first.
2. Executing AT+QMAP="DMZ"[,<enable>,<IP_family>[,<IP_address>]] writes data to NVM.
Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 232 / 282

5G Module Series
Example
AT+QMAP="DMZ" //Query the current setting of DMZ.
+QMAP: "DMZ",0,4
+QMAP: "DMZ",0,6
OK
AT+QMAP="DMZ",1,4,192.168.225.50 //Enable DMZ of IPv4 and the address is 192.168.225.50.
OK
AT+QMAP="DMZ",0,4 //Disable DMZ of IPv4.
OK
12.4. AT+QMAP="GRE" Query/Set GRE Data Acceleration
This command queries or configures GRE data acceleration.
AT+QMAP="GRE" Query/Set GRE Data Acceleration
Write Command Response
AT+QMAP="GRE"[,<enable>[,<IP_ If the optional parameters are omitted, query the current setting:
address1>[,<IP_address2>[,…]]]] +QMAP: "GRE",<enable>[,<IP_address1>[,<IP_address2>
[,…]]]
OK
If any of the optional parameters is specified, set GRE data
acceleration and the IP address of GRE server:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configurations are saved automatically.
Parameter
<enable> Integer type. Enable or disable GRE data acceleration.
0 Disable
1 Enable
<IP_address> String type. IP address of GRE server. It is valid only when <enable> is 1.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 233 / 282

5G Module Series
⚫
NOTE
1. After the GRE data acceleration function is enabled, if you want to change/add the IP address of
GRE server, disable the function first (AT+QMAP="GRE",0), then you can configure a new IP
address.
2. For modules with firmware versions containing "R01" and "R02", you can only execute
AT+QMAP="GRE"[,<enable>[,<IP_address>]] to configure one IP address.
3. For modules with firmware version containing "R03", you can execute
AT+QMAP="GRE"[,<enable>[,<IP_address1>[,<IP_address2>…]]] to set multiple GRE server
IP addresses (maximum ten GRE server IP addresses can be set).
4. Executing AT+QMAP="GRE"[,<enable>[,<IP_address1>[,<IP_address2>…]]] writes data to
NVM. Please proceed with caution.
Example
AT+QMAP="GRE" //Query the current setting of GRE data acceleration.
+QMAP: "GRE",0
OK
AT+QMAP="GRE",1,192.168.2.1 //Enable GRE data acceleration and the address is 192.168.2.1.
OK
AT+QMAP="GRE" //Query the current setting of GRE data acceleration.
+QMAP: "GRE",1,192.168.2.1
OK
12.5. AT+QMAP="LAN" Query/Lock Single IP Address for Default LAN
Interface
This command queries or locks the single IP address for the default LAN interface (VLAN0).
AT+QMAP="LAN" Query/Lock Single IP Address for Default LAN Interface
Write Command Response
AT+QMAP="LAN"[,<IP_address>] If the optional parameter is omitted, query the current setting:
+QMAP: "LAN"[,<IP_address>]
OK
If the optional parameter is specified, lock the single IP address
for the default LAN interface:
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 234 / 282

5G Module Series
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
String type. Dotted decimal IP address without double quotes. Single IP address
<IP_address>
of the default LAN interface.
NOTE
1. <IP_address> must belong to the network segment of the current default LAN interface. The
segment of the LAN interface is 192.168.225.x by default.
2. After a successful configuration, only the IP address specified by <IP_address> can be assigned
under the default LAN interface.
3. Executing AT+QMAP="LAN"[,<IP_address>] writes data to NVM. Please proceed with caution.
Example
AT+QMAP="LAN" //Query the current setting.
+QMAP: "LAN"
OK
AT+QMAP="LAN",192.168.225.50 //Lock the single IP address for the default LAN interface.
OK
AT+QMAP="LAN" //Query the current setting.
+QMAP: "LAN",192.168.225.50
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 235 / 282

5G Module Series
12.6. AT+QMAP="LANIP" Query/Modify DHCP Address Pool of Default
LAN Interface
This command queries or modifies DHCP address pool of the default LAN interface (VLAN0).
AT+QMAP="LANIP" Query/Modify DHCP Address Pool of Default LAN Interface
Write Command Response
AT+QMAP="LANIP"[,<LAN_I If the optional parameters are omitted, query the current setting:
P_start_address>,<LAN_IP_ +QMAP: "LANIP",<LAN_IP_start_address>,<LAN_IP_end_addres
end_address>,<GW_IP_addr s>,<GW_IP_address>
ess>[,<effect>]]
OK
If any of the optional parameters is specified, set DHCP address pool
of the default LAN interface:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Whether the command takes effect immediately depends on <effect>.
Characteristics
The configurations are saved automatically.
Parameter
<LAN_IP_start_address> String type. Start address of DHCP address pool of default LAN
interface. Format: dotted decimal IPv4 address without double quotes.
<LAN_IP_end_address> String type. End address of DHCP address pool of default LAN interface.
Format: dotted decimal IPv4 address without double quotes.
<GW_IP_address> String type. Gateway address of DHCP address pool of default LAN
interface. Format: dotted decimal IPv4 address without double quotes.
<effect> Integer type. Whether the command takes effect immediately or not.
0 Take effect after the module reboots
1 Take effect immediately
NOTE
Executing
AT+QMAP="LANIP"[,<LAN_IP_start_address>,<LAN_IP_end_address>,<GW_IP_address>[,<effect>]]
writes data to NVM. Please proceed with caution.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 236 / 282

5G Module Series
Example
AT+QMAP="LANIP" //Query the current setting of DHCP address pool.
+QMAP: "LANIP",192.168.225.40,192.168.225.60,192.168.225.1
OK
//Set the DHCP address pool and the setting takes effect immediately.
AT+QMAP="LANIP",192.168.111.20,192.168.111.60,192.168.111.1,1
OK
//Set the DHCP address pool and the setting takes effect after the module reboots.
AT+QMAP="LANIP",192.168.111.20,192.168.111.60,192.168.111.1
OK
12.7. AT+QMAP="VLAN" Query/Set VLAN
This command queries or sets VLAN of the module, including enabling or disabling VLAN and querying
current enabled VLAN.
AT+QMAP="VLAN" Query/Set VLAN
Write Command Response
AT+QMAP="VLAN"[,<VLAN_ID> If the optional parameters are omitted, query the enabled VLAN:
,<enable>[,<VLAN_type>]] +QMAP: "VLAN",0
+QMAP: "VLAN",<VLAN_ID1>,<VLAN_type1>
[[+QMAP: "VLAN",<VLAN_ID2>,<VLAN_type2>]
[…]]
OK
If any of the optional parameters is specified, enable or disable the
specified VLAN:
OK
If there is any error:
ERROR
Maximum Response Time 5 s
See the note below for whether the command takes effect
Characteristics immediately or not.
The configurations are saved automatically.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 237 / 282

5G Module Series
Parameter
<VLAN_ID> Integer type. VLAN ID. Range: 0, 2–255.
0 is displayed only in the response string and indicates the physical default
LAN interface rather than a VLAN ID.
<enable> String type. Enable or disable VLAN specified by <VLAN_ID>.
"enable" Enable
"disable" Disable
<VLAN_type> Integer type. VLAN type. It is valid only when <enable> is "enable".
1 ETH
2 ECM
3 RNDIS
11 ETH without enabling VLAN data acceleration
12 ECM without enabling VLAN data acceleration
13 RNDIS without enabling VLAN data acceleration
NOTE
1. If <VLAN_type>=1/2/3, the module reboots automatically when you enable the first VLAN of any
type or disable the last VLAN of the specified type.
2. In other conditions, VLAN enabling or disabling takes effect immediately and the module does not
reboot automatically.
3. Executing AT+QMAP="VLAN"[,<VLAN_ID>,<enable>[,<VLAN_type>]] writes data to NVM.
Please proceed with caution.
Example
AT+QMAP="VLAN" //Query the list of enabled VLAN IDs.
+QMAP: "VLAN",0
+QMAP: "VLAN",2,1 //VLAN 2 (eth0.2) of ETH is enabled.
+QMAP: "VLAN",3,1 ///VLAN 3 (eth0.3) of ETH is enabled.
OK
AT+QMAP="VLAN",4,"enable",1 //Enable VLAN 4 (eth0.4) of ETH.
OK
AT+QMAP="VLAN",4,"disable" //Disable VLAN 4 (eth0.4) of ETH.
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 238 / 282

5G Module Series
12.8. AT+QMAP="MPDN_rule" Query/Modify QMAP Multiple Data Call
Rule
This command queries or modifies the QMAP multiple data call rules.
AT+QMAP="MPDN_rule" Query/Modify QMAP Multiple Data Call Rule
Write Command Response
AT+QMAP="MPDN_rule"[,<rule_nu If the optional parameters are omitted, query the current
m>[,<profileID>,<VLAN_ID>,<IPPT_ setting:
mode>,<auto_connect>[,<IPPT_info +QMAP: "MPDN_rule",<rule_num>,<profileID>,<VLAN_I
>]]] D>,<IPPT_mode>,<auto_connect>
+QMAP: "MPDN_rule",<rule_num>,<profileID>,<VLAN_I
D>,<IPPT_mode>,<auto_connect>
+QMAP: "MPDN_rule",<rule_num>,<profileID>,<VLAN_I
D>,<IPPT_mode>,<auto_connect>
+QMAP: "MPDN_rule",<rule_num>,<profileID>,<VLAN_I
D>,<IPPT_mode>,<auto_connect>
OK
If only <rule_num> is specified, disable a specified QMAP
data call rule:
OK
If any of the optional parameters is specified, set the specified
QMAP data call rule:
OK
If there is any error:
ERROR
Maximum Response Time 5 s
See the note below for whether the command takes effect
Characteristics immediately or not.
The configurations are saved automatically.
Parameter
<rule_num> Integer type. Rule ID of QMAP multiple data call. Range: 0–3.
<profileID> Integer type. APN profile ID used by QMAP data call rule.
Range: 1–16.
<VLAN_ID> Integer type. VLAN ID used by QMAP data call rule.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 239 / 282

5G Module Series
Range: 0, 2–4094.
0 is displayed only in the response string and indicates physical default LAN
interface rather than a VLAN ID.
<IPPT_mode> Integer type. Enable or disable IPPT mode (IP Passthrough mode) in QMAP data
call rule.
0 Disable IPPT mode
1 Enable IPPT mode (ETH)
2 Enable IPPT mode (Wi-Fi)
3 Enable IPPT mode (USB-ECM/RNDIS)
4 Enable IPPT mode (Any Device)
5 Enable IPPT mode (ETH-NIC2, supported only on RG520N Series,
RG525F-NA, RG520F Series, RG530F Series, RM520N Series and
RM530N-GL modules.)
<auto_connect> Integer type. Enable or disable automatic connecting in QMAP data call rule.
1 Enable
0 Disable
<IPPT_info> String type.
If <IPPT_mode> is 0, <IPPT_info> does not need to be filled in.
If <IPPT_mode> is 1, <IPPT_info> is the peer NIC MAC address bound in IPPT
mode.
⚫ If <IPPT_info> is set to "FF:FF:FF:FF:FF:FF", the module will always deliver
the public network address to the newly connected ETH device.
⚫ If <IPPT_info> is set to "00:00:00:00:00:00", the module will only deliver the
public network address to the first connected ETH device.
⚫ If <IPPT_info> is set to the MAC address of an ethernet device, the module
will only deliver the public network address to the ETH device.
If <IPPT_mode> is 2, <IPPT_info> is the peer NIC MAC address bound in IPPT
mode.
⚫ If <IPPT_info> is set to "FF:FF:FF:FF:FF:FF", the module will always deliver
the public network address to the newly connected Wi-Fi device.
⚫ If <IPPT_info> is set to "00:00:00:00:00:00", the module will only deliver the
public network address to the first connected Wi-Fi device.
⚫ If <IPPT_info> is set to the MAC address of a Wi-Fi device, the module will
only deliver the public network address to the Wi-Fi device.
If <IPPT_mode> is 3, <IPPT_info> is the peer host name bound in IPPT mode.
⚫ If <IPPT_info > is set to "FF:FF:FF:FF:FF:FF", the module will always deliver
the public network address to the newly connected USB device.
⚫ If <IPPT_info> is set to "00:00:00:00:00:00", the module will only deliver the
public network address to the first connected USB device.
⚫ When using the first two methods, please ensure that the NIC MAC address of
the host's USB network card [ECM/RNDIS] remains unchanged, otherwise the
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 240 / 282

5G Module Series
module will consider it a different device.
⚫ If <IPPT_info> is set to the hostname of a USB device, the module will only
deliver the public network address to the USB device.
If <IPPT_mode> is 4, <IPPT_info> means that the module can deliver the public IP
address to any device with any interface type. The value can only be set as follows:
⚫ If <IPPT_info> is set to "FF:FF:FF:FF:FF:FF", the module will always deliver
the public IP address to the latest connected device of any interface type.
⚫ If <IPPT_info> is set to "00:00:00:00:00:00", the module will only deliver the
public network address to the first connected device of any interface type.
⚫ In this case <IPPT_info> cannot be set to other values.
If <IPPT_mode> is 5, <IPPT_info> is the peer NIC MAC address bound in IPPT
mode.
⚫ If <IPPT_info> is set to "FF:FF:FF:FF:FF:FF", the module will always deliver
the public network address to the newly connected ETH-NIC2 device.
⚫ If <IPPT_info> is set to "00:00:00:00:00:00", the module will only deliver the
public network address to the first connected ETH-NIC2 device.
⚫ If <IPPT_info> is set to the MAC address of an ethernet device, the module
will only deliver the public network address to the ETH-NIC2 device.
When IPPT mode is enabled,
⚫ If the IPPT NAT working mode is WithNAT (AT+QMAP="IPPT_NAT",1), the
LAN device specified by <IPPT_info> will obtain the public network address,
other LAN devices will obtain the private network address, and the module will
perform network address translation on all LAN device data.
⚫ If the IPPT NAT working mode is WithoutNAT (AT+QMAP="IPPT_NAT",0), the
LAN device specified by <IPPT_info> will obtain the public network address,
and the module will not perform network address translation on the data of the
LAN device, and other LAN devices will not obtain any IP addresses. In
addition, in this mode, the IPPT function, as applied to the latest devices, will
be invalid, and "FF:FF:FF:FF:FF:FF" will be treated as equivalent to
"00:00:00:00:00:00".
NOTE
1. If only the physical default LAN interface is required to access network and there is no need to
support QMAP multiple data call, you should set <rule_num>=0 and <VLAN_ID>=0.
2. The QMAP multiple data call is implemented by binding the WAN interfaces obtained from data
calls of different APNs to the LAN/VLAN interface, and implementing the NAT configuration
between the corresponding WAN and LAN/VLAN. In this way, the devices under different
LAN/VLAN interfaces can access different network through the corresponding WAN interface.
3. When configuring QMAP data call rule, if you need to use a VLAN interface (<VLAN_ID> is not
0), you need to create a corresponding VLAN interface through AT+QMAP="VLAN" first.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 241 / 282

5G Module Series
4. IPPT mode (IP Passthrough mode), is a function of transparently transmitting the IP address
(Public IP) assigned by the operator to the LAN device.
5. By default, when using a USB (ECM/RNDIS) interface to start a QMAP data call, if the IPPT
mode is enabled, you need to set <IPPT_mode> to 3, and set the hostname of the LAN device
in <IPPT_info>. In most cases, the MAC address of the USB virtual Ethernet interface
(ECM/RNDIS) is not fixed. However, the module supports IPPT mode by setting <IPPT_mode>
to 1 and setting the MAC address of the LAN USB device in <IPPT_info> in actual use.
6. WLAN interface does not support VLAN function, WLAN belongs to VLAN0. In actual use, to
assign the public IP to the WLAN device, you need to set <IPPT_mode> to 2, and <VLAN_ID>
can only be 0.
7. By default, the data call initiated with the first rule (<rule_num>=0) is the default QMAP data
call.
8. The default QMAP data call is bound to the physical LAN interface (VLAN0) by default. If you
change the bound LAN/VLAN interface of the default QMAP data call, the module reboots
automatically. For example, execute AT+QMAP="MPDN_rule",0,1,2,0,1 (bind the default
QMAP data call rule to <VLAN_ID>=2). If AT+QMAP="MPDN_rule",0 is executed to disable
the default QMAP data call rule, the LAN/VLAN interface bound to the default QMAP data call
rule automatically changes the physical LAN interface from <VLAN_ID>=2, and the module
reboots automatically.
9. The module accesses the network through the data connection initiated by the default QMAP
data call rule. That is, if <rule_num>=0 does not initiate a data connection, the module cannot
access network.
10. Executing AT+QMAP="MPDN_rule"[,<rule_num>[,<profileID>,<VLAN_ID>,<IPPT_mode>,<a
uto_connect>[,<IPPT_info>]]] writes data to NVM. Please proceed with caution.
Example
AT+QMAP="MPDN_rule" //Query the current QMAP data call rules.
+QMAP: "MPDN_rule",0,0,0,0,0
+QMAP: "MPDN_rule",1,0,0,0,0
+QMAP: "MPDN_rule",2,0,0,0,0
+QMAP: "MPDN_rule",3,0,0,0,0
OK
AT+QMAP="MPDN_rule",0,1,0,0,1 //Configure and enable QMAP data call rule 0
OK
AT+QMAP="MPDN_rule",1,5,2,0,1 //Configure and enable QMAP data call rule 1.
OK
AT+QMAP="MPDN_rule" //Query the current QMAP data call rules.
+QMAP: "MPDN_rule",0,1,0,0,1
+QMAP: "MPDN_rule",1,5,2,0,1
+QMAP: "MPDN_rule",2,0,0,0,0
+QMAP: "MPDN_rule",3,0,0,0,0
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 242 / 282

5G Module Series
OK
AT+QMAP="MPDN_rule",1 //Disable QMAP data call rule 1.
OK
AT+QMAP="MPDN_rule" //Query the current QMAP data call rules.
+QMAP: "MPDN_rule",0,1,0,0,1
+QMAP: "MPDN_rule",1,0,0,0,0
+QMAP: "MPDN_rule",2,0,0,0,0
+QMAP: "MPDN_rule",3,0,0,0,0
OK
12.9. AT+QMAP="IPPT_NAT" Query/Set IPPT NAT Working Mode of
QMAP Data Call
This command queries or configures whether to use NAT (Network Address Translation) in IPPT mode.
AT+QMAP="IPPT_NAT" Query/Set IPPT NAT Working Mode of QMAP Data Call
Write Command Response
AT+QMAP="IPPT_NAT"[,<IPPT_NAT If the optional parameter is omitted, query the current setting:
>] +QMAP: "IPPT_NAT",<IPPT_NAT>
OK
If the optional parameter is specified, set IPPT NAT working
mode:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<IPPT_NAT> Integer type. IPPT NAT working mode.
0 WihoutNAT. NAT is not used in IPPT mode.
1 WithNAT. NAT is used in IPPT mode.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 243 / 282

5G Module Series
NOTE
1. Changing IPPT NAT working mode disconnects all QMAP data call connections. The disconnected
QMAP data call can be reconnected automatically if automatic connecting is enabled. If it is
disabled, manually execute AT+QMAP="connect" to start a QMAP data call after changing IPPT
NAT working mode.
2. If you change the IPPT NAT working mode to WithoutNAT from WithNAT, the IPPT modes
configured in all QMAP data call rules change to WithoutNAT automatically. If you change the IPPT
NAT working mode to WithNAT from WithoutNAT, the IPPT modes configured in all QMAP data call
rules change to WithNAT automatically.
3. Executing AT+QMAP="IPPT_NAT" writes data to NVM. Please proceed with caution.
Example
AT+QMAP="IPPT_NAT" //Query current setting.
+QMAP: "IPPT_NAT",0
OK
AT+QMAP="IPPT_NAT",1 //Set to using NAT in IPPT mode.
OK
12.10. AT+QMAP="connect" Initiate/Terminate QMAP Data Call
This command initiates or terminates a QMAP data call.
AT+QMAP="connect" Initiates/Terminates QMAP Data Call
Write Command Response
AT+QMAP="connect",<rule_num>,<co OK
nnect> Or
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<rule_num> Integer type. Rule ID of QMAP multiple data call. Range: 0–3.
<connect> Integer type. Initiate or terminate QMAP data call.
0 Terminate
1 Initiate
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 244 / 282

5G Module Series
NOTE
1. If <auto_connect>=1 (see AT+QMAP="MPDN_rule"), the specified QMAP data call rule initiates
an automatic data call, and you cannot initiate or terminate this data connection over
AT+QMAP="connect". If you want to control QMAP data call manually with
AT+QMAP="connect", you should disable automatic connecting in the rule with
AT+QMAP="MPDN_rule".
2. Executing AT+QMAP="connect",<rule_num>,<connect> writes data to NVM. Please proceed
with caution.
Example
AT+QMAP="connect",0,1 //Initiate QMAP data call of rule 0.
OK
AT+QMAP="connect",0,0 //Terminate QMAP data call of rule 0.
OK
12.11. AT+QMAP="auto_connect" Query/Modify Automatic Connection
of QMAP Data Call
This command queries or modifies automatic connection of QMAP data call.
AT+QMAP="auto_connect" Query/Modify Automatic Connection of QMAP Data Call
Write Command Response
AT+QMAP="auto_connect"[,<rul If the optional parameters are omitted, query the current settings
e_num>[,<auto_connect>[,<profi of all QMAP data call rules:
leID>]]] +QMAP: "auto_connect",<rule_num>,<auto_connect>
+QMAP: "auto_connect",<rule_num>,<auto_connect>
+QMAP: "auto_connect",<rule_num>,<auto_connect>
+QMAP: "auto_connect",<rule_num>,<auto_connect>
OK
If only <rule_num> is specified, query the current setting of the
specified QMAP data call rule:
+QMAP: "auto_connect",<rule_num>,<auto_connect>
OK
If only <rule_num> and <auto_connect> are specified, enable
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 245 / 282

5G Module Series
or disable automatic connecting for the specified QMAP data call
rule:
OK
If any of the optional parameters is specified, enable automatic
connecting and set the APN Profile ID, or disable automatic
connecting for the specified QMAP data call rule:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Parameter
<rule_num> Integer type. Rule ID of QMAP multiple data call. Range: 0–3.
<auto_connect> Integer type. Enable or disable automatic connection in QMAP data call.
0 Disable
1 Enable
<profileID> Integer type. APN Profile ID used by QMAP data call rule. Range: 1–16.
NOTE
1. Before modifying <auto_connect> of the specified QMAP data call rule, first ensure that the
specified rule has been configured and enabled with AT+QMAP="MPDN_rule".
2. Executing AT+QMAP="auto_connect"[,<rule_num>[,<auto_connect>[,<profileID>]]] writes data
to NVM. Please proceed with caution.
Example
AT+QMAP="auto_connect" //Query the current setting.
+QMAP: "auto_connect",0,1
+QMAP: "auto_connect",1,0
+QMAP: "auto_connect",2,0
+QMAP: "auto_connect",3,0
OK
AT+QMAP="auto_connect",0 //Query automatic connection of rule 0.
+QMAP: "auto_connect",0,1
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 246 / 282

5G Module Series
OK
AT+QMAP="auto_connect",1,1 //Set automatic connection of rule 2.
OK
AT+QMAP="auto_connect",2,1,6 //Set automatic connection of rule 2 and modify
<profileID> to 6.
OK
12.12. AT+QMAP="MPDN_status" Query QMAP Multiple Data Call Status
This command queries status of QMAP multiple data call.
AT+QMAP="MPDN_status" Query QMAP Multiple Data Call Status
Write Command Response
AT+QMAP="MPDN_status" +QMAP: "MPDN_status",<rule_num>,<profileID>,<IPPT_st
atus>,<connect_status>
+QMAP: "MPDN_status",<rule_num>,<profileID>,<IPPT_st
atus>,<connect_status>
+QMAP: "MPDN_status",<rule_num>,<profileID>,<IPPT_st
atus>,<connect_status>
+QMAP: "MPDN_status",<rule_num>,<profileID>,<IPPT_st
atus>,<connect_status>
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics -
Parameter
<rule_num> Integer type. Rule ID of QMAP multiple data call. Range: 0–3.
<profileID> Integer type. APN profile ID used by QMAP data call rule. Range: 1–16.
<IPPT_status> Integer type. Whether IPPT mode is enabled in QMAP data call rule.
0 Enabled
1 Disabled
<connect_status> Integer type. Status of QMAP data call.
0 Disconnected
1 Connected
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 247 / 282

5G Module Series
Example
AT+QMAP="MPDN_status" // Query status of QMAP multiple data call.
+QMAP: "MPDN_status",0,1,1,1
+QMAP: "MPDN_status",1,2,0,1
+QMAP: "MPDN_status",2,3,0,0
+QMAP: "MPDN_status",3,0,0,0
OK
12.13. AT+QMAP="SFE" Query/Set SFE Software Acceleration
This command queries or sets software acceleration of the module.
AT+QMAP="SFE" Query/Set SFE Software Acceleration
Write Command Response
AT+QMAP="SFE"[,<status>] If the optional parameter is omitted, query the current setting:
+QMAP: "SFE",<status>
OK
If the optional parameter is specified, enable or disable SFE
software acceleration:
OK
If there is any error:
ERROR
Maximum Response Time 500 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<status> String type. Enable or disable SFE software acceleration.
"enable" Enable
"disable" Disable
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 248 / 282

5G Module Series
NOTE
1. Only if the module does not support hardware acceleration (IPA), the SFE function can provide
limited performance optimization. If the module supports hardware acceleration (IPA), this function
is invalid.
2. Executing AT+QMAP="SFE"[,<status>] writes data to NVM. Please proceed with caution.
Example
AT+QMAP="SFE" //Query current setting.
+QMAP: "SFE","disable"
OK
AT+QMAP="SFE","enable" //Enable SFE software acceleration.
OK
12.14. AT+QMAP="domain" Query/Set Gateway Domain Name of
LAN/VLAN Interface
This command queries or configures gateway domain name of LAN/VLAN interface.
AT+QMAP="domain" Query/Set Gateway Domain Name of LAN/VLAN Interface
Write Command Response
AT+QMAP="domain"[,<domain_na If the optional parameter is omitted, query the current setting:
me>] +QMAP: "domain",<domain_name>
OK
If the optional parameter is specified, set gateway domain
name of LAN/VLAN interface:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 249 / 282

5G Module Series
Parameter
<domain_name> String type. LAN/VLAN gateway domain name. For example,
"www.example.com".
NOTE
Executing AT+QMAP="domain"[,<domain_name>] writes data to NVM. Please proceed with caution.
Example
AT+QMAP="domain" //Query gateway domain name of LAN/VLAN interface.
+QMAP: "domain","www.example.com"
OK
AT+QMAP="domain","www.example.com" //Set gateway domain name of LAN/VLAN interface.
OK
12.15. AT+QMAP="DHCPV6DNS" Query/Set IPv6 DNS of QMAP Data Call
This command queries or configures IPv6 DNS of QMAP data call.
AT+QMAP="DHCPV6DNS" Query/Set IPv6 DNS of QMAP Data Call
Write Command Response
AT+QMAP="DHCPV6DNS"[,<stat If the optional parameter is omitted, query the current setting:
us>] +QMAP: "DHCPV6DNS",<status>
OK
If the optional parameter is specified, enable or disable IPv6
DNS:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 250 / 282

5G Module Series
Parameter
<status> String type. Enable or disable IPv6 DNS.
"enable" Enable
"disable" Disable
NOTE
Executing AT+QMAP="DHCPV6DNS"[,<status>] writes data to NVM. Please proceed with caution.
Example
AT+QMAP="DHCPV6DNS" //Query current setting
+QMAP: "DHCPV6DNS","disable"
OK
AT+QMAP="DHCPV6DNS","enable" //Enable IPv6 DNS
OK
12.16. AT+QMAP="DHCPV4DNS" Query/Set IPv4 DNS Proxy of QMAP
Data Call
This command queries or sets IPv4 DNS proxy of QMAP data call.
AT+QMAP="DHCPV4DNS" Query/Set IPv4 DNS Proxy of QMAP Data Call
Write Command Response
AT+QMAP="DHCPV4DNS"[,<stat If the optional parameter is omitted, query the current setting:
us>] +QMAP: "DHCPV4DNS",<status>
OK
If the optional parameter is specified, enable or disable IPv4
DNS:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics The command takes effect after the module is rebooted.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 251 / 282

5G Module Series
The configuration is saved automatically.
Parameter
<status> String type. Enable or disable IPv4 DNS.
"enable" Enable
"disable" Disable
NOTE
1. After enabling the IPv4 DNS proxy function, all LAN ports will restart. If multiple data-calls are set
up, multiple restarts may occur. To obtain the new DNS parameters, the host needs to send DHCP
packets.
2. Executing AT+QMAP="DHCPV4DNS"[,<status>] writes data to NVM. Please proceed with
caution.
Example
AT+QMAP="DHCPV4DNS" //Query current setting
+QMAP: "DHCPV4DNS","enable"
OK
AT+QMAP="DHCPV4DNS","disable" //Disable IPv4 DNS
OK
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 252 / 282

5G Module Series
13
Appendix References
13.1. Terms and Abbreviations
Table 5: Terms and Abbreviations
Abbreviation Description
3GPP 3rd Generation Partnership Project
5GCN 5G Core Network
5GS 5G System
ADC Analog To Digital Converter
AP Application Processor
APDU Application Protocol Data Unit
APN Access Point Name
ARFCN Absolute Radio-Frequency Channel Number
ARM Advanced RISC (Reduced Instruction Set Computing) Machine
ASCII American Standard Code for Information Interchange
BB Baseband
BCD Binary Coded Decimal
BER Bit Error Rate
BT Bluetooth
CA Carrier Aggregation
CBM Cell Broadcast Message
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 253 / 282

5G Module Series
CDRX Connected Discontinuous Reception
CFU Call Forwarding Unconditional
CLI Calling Line Identification
CLIP Calling Line Identification Presentation
CLIR Calling Line Identification Restriction
COL Connected Line
COLP Connected Line Identification Presentation
COLR Connected Line Identification Restriction
CQI Channel Quality Indicator
CS Circuit Switch
CSD Circuit Switch Data
CSI Channel State Information
CUG Closed User Group
DCE Data Communication Equipment
DCS Data Coding Scheme
DF Dedicated File
DHCP Dynamic Host Configuration Protocol
DL Downlink
DNS Domain Name Server
DPCH Dedicated Physical Channel
DPR Dynamic Power Reduction
DSS Dynamic Spectrum Sharing
DTE Data Terminal Equipment
DTMF Dual-Tone Multifrequency
DTR Data Terminal Ready
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 254 / 282

5G Module Series
EARFCN E-UTRA Absolute Radio Frequency Channel Number
ECC Emergency Communications Center
ECGI E-UTRAN Cell Global Identifier
ECI E-UTRAN Cell Identifier
ECM Ethernet Control Model
ECT Explicit Call Transfer supplementary service
EFS Encrypting File System
eMLPP Enhanced Multi-Level Precedence and Pre-emption Service
EN-DC E-UTRA NR Dual Connectivity
EPS Evolved Packet System
ETH Ethernet
eUTRAN Evolved Universal Terrestrial Radio Access Network
FDD Frequency Division Duplex
FDPCH Fraction-Dedicated Physical Channel
FOTA Firmware Upgrade Over-The-Air
GERAN GSM/EDGE Radio Access Network
GGSN Gateway GPRS Support Node
GMT Greenwich Mean Time
GRE Generic Routing Encapsulation
GPIO General-Purpose Input/Output
GPRS General Packet Radio Service
GPS Global Positioning System
GSM Global System for Mobile Communications
HLR Home Location Register
HSDPA High Speed Downlink Packet Access
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 255 / 282

5G Module Series
HSUPA High Speed Uplink Packet Access
ICCID Integrated Circuit Card ID
IMEI International Mobile Equipment Identity
IMS IP Multimedia Subsystem
IMSI International Mobile Subscriber Identity
IPv4 Internet Protocol version 4
IPv6 Internet Protocol version 6
IRA International Reference Alphabet
ISDN Integrated Services Digital Network
iSIM IP Multimedia Service Identity Module
IWF Interworking Function
LAN Local Area Network
LLC Logical Link Control
LTE Long-Term Evolution
MAC Medium Access Control
MCC Mobile Country Code
ME Mobile Equipment
MNC Mobile Network Code
MO Mobile Original
MPTY MultiParty
MS Mobile Station
MSC Mobile Switching Center
MSISDN Mobile Subscriber International Integrated Service Digital Network number
MT Mobile Terminal
MTU Maximum Transmission Unit
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 256 / 282

5G Module Series
NAS Non-Access Stratum
NAT Network Address Translation
NCI NR Cell Identifier
NCGI NR Cell Global Identifier
NG-RAN Next-Generation Radio Access Network
NIC Network Interface Controller
NITZ Network Identity and Time Zone / Network Informed Time Zone
NR New Radio
NSA Non-Standalone
NSAPI Network Service Access Point Identifier
NSSAI Network Slice Selection Assistance Information
NTC Negative Temperature Coefficient
NVM Non-Volatile Memory
OIR Originating Identification Restriction
PCIe Peripheral Component Interconnect Express
PCIe EP PCI Express Endpoint Device
PCIe RC PCI Express Root Complex
PCO Protocol Configuration Options
PDN Public Data Network
PDP Packet Data Protocol
PDU Protocol Data Unit
PIN Personal Identification Number
PLMN Public Land Mobile Network
PMU Power Management Unit
PPP Point-to-Point Protocol
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 257 / 282

5G Module Series
PS Packet Switch
PSC Primary Synchronization Code
PUK PIN Unlock Key
QoS Quality of Service
RAN Radio Access Network
RAT Radio Access Technology
RF Radio Frequency
RI Ring Indicator
RLP Radio Link Protocol
RNDIS Remote Network Driver Interface Specification
RP Relay Protocol
RRC Radio Resource Control
RSRP Reference Signal Received Power
RSRQ Reference Signal Received Quality
RSSI Received Signal Strength Indicator
RTC Real-Time Clock
SA Standalone
SINR Signal to Interference plus Noise Ratio
SLIC Subscriber Line Interface Circuit
SMS Short Messaging Service
SMSC Short Message Service Center
SNDCP Sub Network Dependence Convergence Protocol
S-NSSAI Single Network Slice Selection Assistance Information
SSC Session and Service Continuity
SST Slice/Service Type
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 258 / 282

5G Module Series
TA Terminal Adapter
TDD Time Division Duplex
TFT Traffic Flow Template
TPDU Transport Protocol Data Unit
UART Universal Asynchronous Receiver/Transmitter
UCS2 Universal Character Set (UCS-2) Format
UDUB User Determined User Busy
UE User Equipment
UICC Universal Integrated Circuit Card
UIM User Identity Model
UL Uplink
UMTS Universal Mobile Telecommunications System
URC Unsolicited Result Code
USB Universal Serial Bus
USSD Unstructured Supplementary Service Data
(U)SIM (Universal) Subscriber Identity Module
UTRA UMTS Terrestrial Radio Access
UTRAN Universal Terrestrial Radio Access Network
VLAN Virtual Local Area Network
VLR Visitor Location Register
WCDMA Wideband Code Division Multiple Access
WIM Wireless Identity Module
WLAN Wireless Local Area Network
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 259 / 282

                                          5G Module Series
13.2. Factory Default Settings Restorable with AT&F

Table 6: Factory Default Settings Restorable with AT&F
| AT Command  | Parameter                 | Factory Default  |
| ----------- | ------------------------- | ---------------- |
| ATE         | <value>                   | 1                |
| ATQ         | <n>                       | 0                |
| ATS0        | <n>                       | 0                |
| ATS3        | <n>                       | 13               |
| ATS4        | <n>                       | 10               |
| ATS5        | <n>                       | 8                |
| ATS6        | <n>                       | 2                |
| ATS7        | <n>                       | 0                |
| ATS8        | <n>                       | 2                |
| ATS10       | <n>                       | 15               |
| ATV         | <value>                   | 1                |
| ATX         | <value>                   | 4                |
| AT+CREG     | <n>                       | 0                |
| AT+CGREG    | <n>                       | 0                |
| AT+CMEE     | <n>                       | 1                |
| AT+CSCS     | <chset>                   | "GSM"            |
| AT+CSTA     | <type>                    | 129              |
| AT+CR       | <mode>                    | 0                |
| AT+CRC      | <mode>                    | 0                |
| AT+CSMS     | <service>,<mt>,<mo>,<bm>  | 0,1,1,1          |
| AT+CMGF     | <mode>                    | 0                |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                260 / 282

                                          5G Module Series
| AT+CSMP    | <fo>,<vp>,<pid>,<dcs>        | 17,167,0,0      |
| ---------- | ---------------------------- | --------------- |
| AT+CSDH    | <show>                       | 0               |
| AT+CSCB    | <mode>,<mids>,<dcss>         | 0,"",""         |
| AT+CPMS    | <mem1>,<mem2>,<mem3>         | "ME","ME","ME"  |
| AT+CNMI    | <mode>,<mt>,<bm>,<ds>,<bfr>  | 2,1,0,0,0       |
| AT+CMMS    | <n>                          | 0               |
| AT+CVHU    | <mode>                       | 0               |
| AT+CLIP    | <n>                          | 0               |
| AT+COLP    | <n>                          | 0               |
| AT+CLIR    | <n>                          | 0               |
| AT+CSSN    | <n><m>                       | 0,0             |
| AT+CTZR    | <reporting>                  | 0               |
| AT+CPBS    | <storage>                    | "SM"            |
| AT+CGEREP  | <mode>,<brf>                 | 0,0             |
| AT+CEREG   | <n>                          | 0               |
| AT+CCWA    | <n>                          | 0               |
| AT+CUSD    | <mode>                       | 0               |

13.3. AT Command Settings Storable with AT&W

Table 7: AT Command Settings Storable with AT&W
| AT Command  | Parameters  | Display with AT&V  |
| ----------- | ----------- | ------------------ |
| ATE         | <value>     | Yes                |
| ATQ         | <n>         | Yes                |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                261 / 282

                                          5G Module Series
| ATS0      | <n>      | Yes  |
| --------- | -------- | ---- |
| ATS7      | <n>      | Yes  |
| ATS10     | <n>      | Yes  |
| ATV       | <value>  | Yes  |
| ATX       | <value>  | Yes  |
| AT+CREG   | <n>      | No   |
| AT+CGREG  | <n>      | No   |
| AT+CEREG  | <n>      | No   |

13.4. AT Command Settings Storable with ATZ

Table 8: AT Command Settings Storable with ATZ
| AT Command  | Parameter  | Factory Default  |
| ----------- | ---------- | ---------------- |
| ATE         | <value>    | 1                |
| ATQ         | <n>        | 0                |
| ATS0        | <n>        | 0                |
| ATS7        | <n>        | 0                |
| ATS10       | <n>        | 15               |
| ATV         | <value>    | 1                |
| ATX         | <value>    | 4                |
| AT+CREG     | <n>        | 0                |
| AT+CGREG    | <n>        | 0                |
| AT+CEREG    | <n>        | 0                |

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                262 / 282

5G Module Series
13.5. Summary of CME ERROR Codes
Final result code +CME ERROR: <err> indicates an error related to mobile equipment or network. The
operation is similar to ERROR result code. If +CME ERROR: <err> is the result code for any of the
commands in a command line, none of the following commands in the same command line is executed
(neither ERROR nor OK result code should be returned as a result of a completed command line
execution).
<err> values are mostly used by common message commands. The following table lists most general
and GRPS-related ERROR codes. For some GSM protocol failure causes described in GSM
specifications, the corresponding ERROR codes are not included.
Table 9: Summary of General +CME ERROR: <err> Codes
Numeric <err> value Verbose <err> value
0 Phone failure
1 No connection to phone
2 Phone-adaptor link reserved
3 Operation not allowed
4 Operation not supported
5 PH-SIM PIN required
6 PH-FSIM PIN required
7 PH-FSIM PUK required
10 (U)SIM not inserted
11 (U)SIM PIN required
12 (U)SIM PUK required
13 (U)SIM failure
14 (U)SIM busy
15 (U)SIM wrong
16 Incorrect password
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 263 / 282

5G Module Series
17 (U)SIM PIN2 required
18 (U)SIM PUK2 required
20 Memory full
21 Invalid index
22 Not found
23 Memory failure
24 Text string too long
25 Invalid characters in text string
26 Dial string too long
27 Invalid characters in dial string
30 No network service
31 Network timeout
32 Network not allowed - emergency calls only
40 Network personalization PIN required
41 Network personalization PUK required
42 Network subset personalization PIN required
43 Network subset personalization PUK required
44 Service provider personalization PIN required
45 Service provider personalization PUK required
46 Corporate personalization PIN required
47 Corporate personalization PUK required
901 Audio unknown error
902 Audio invalid parameters
903 Audio operation is not supported
904 Audio device is busy
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 264 / 282

5G Module Series
13.6. Summary of CMS ERROR Codes
Final result code +CMS ERROR: <err> indicates an error related to mobile equipment or network. The
operation is similar to ERROR result code. None of the following commands in the same command line
is executed. Neither ERROR nor OK result code should be returned.
<err> values mostly used by common message commands:
Table 10: Summary of General +CMS ERROR: <err> Codes
Code of <err> Meaning
300 ME failure
301 SMS ME reserved
302 Operation not allowed
303 Operation not supported
304 Invalid PDU mode
305 Invalid text mode
310 (U)SIM not inserted
311 (U)SIM pin necessary
312 PH (U)SIM pin necessary
313 (U)SIM failure
314 (U)SIM busy
315 (U)SIM wrong
316 (U)SIM PUK required
317 (U)SIM PIN2 required
318 (U)SIM PUK2 required
320 Memory failure
321 Invalid memory index
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 265 / 282

                                          5G Module Series
| 322  Memory full                       |     |     |
| -------------------------------------- | --- | --- |
| 330  SMSC address unknown              |     |     |
| 331  No network                        |     |     |
| 332  Network timeout                   |     |     |
| 340  Not expected                      |     |     |
| 500  Unknown                           |     |     |
| 512  (U)SIM not ready                  |     |     |
| 513  Message length exceeded           |     |     |
| 514  Invalid request parameters        |     |     |
| 515  ME storage failure                |     |     |
| 517  Invalid service mode              |     |     |
| 528  More message to send state error  |     |     |
| 529  MO SMS is not allowed             |     |     |
| 531  ME storage full                   |     |     |

13.7. Summary of URC

Table 11: Summary of URC
| Index  URC Display  | Meaning                            | Condition  |
| ------------------- | ---------------------------------- | ---------- |
| 1  +QUSIM: 1        | (U)SIM card initialization status  | -          |
+QSIMSTAT: <enable>,<inser
| 2   | (U)SIM card insertion status  | AT+QSIMSTAT=1  |
| --- | ----------------------------- | -------------- |
ted_status>
| 3  +CREG: <stat>  | MT registration status   | AT+CREG=1  |
| ----------------- | ------------------------ | ---------- |
+CREG: <stat>[,<lac>,<ci>[,< MT  network  registration  status  and
| 4   |     | AT+CREG=2  |
| --- | --- | ---------- |
AcT>]]  location information
5  +CGREG: <stat>  MT network registration status  AT+CGREG=1
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                266 / 282

                                          5G Module Series
+CGREG: <stat>[,[<lac>],[<c MT network registration and location
| 6   |     |     |     | AT+CGREG=2  |
| --- | --- | --- | --- | ----------- |
i>],[<AcT>],[<rac>]]  information
| 7  +CTZV: <tz>  | Time zone reporting  |     |     | AT+CTZR=1  |
| --------------- | -------------------- | --- | --- | ---------- |
8  +CTZE: <tz>,<dst>,<time>  Extended time zone reporting  AT+CTZR=2
EPS network registration status
| 9  +CEREG: <stat>  |     |     |     | AT+CEREG=1  |
| ------------------ | --- | --- | --- | ----------- |
change in E-UTRAN
+CEREG: <stat>[,<tac>,<ci>[,
| 10  | Network cell change in E-UTRAN  |     |     | AT+CEREG=2  |
| --- | ------------------------------- | --- | --- | ----------- |
<AcT>]]
Network registration status change in
| 11  +C5GREG: <stat>  |     |     |     | AT+C5GREG=1  |
| -------------------- | --- | --- | --- | ------------ |
5GS
+C5GREG: <stat>[,[<tac>],[<
Network cell change in 5GS or
ci>],[<AcT>],[<Allowed_NSS
| 12  | whether there is a network provided  |     |     | AT+C5GREG=2  |
| --- | ------------------------------------ | --- | --- | ------------ |
AI_length>],[<Allowed_NSSA
an allowed NSSAI
I>]]
New message is received and saved
| 13  +CMTI: <mem>,<index>  |     |     |     | See AT+CNMI  |
| ------------------------- | --- | --- | --- | ------------ |
to memory
+CMT: [<alpha>],<length><C New message is received and output
| 14  |     |     |     | See AT+CNMI  |
| --- | --- | --- | --- | ------------ |
R><LF><pdu>  directly to TE (PDU mode)
+CMT: <oa>,[<alpha>],<scts>
[,<tooa>,<fo>,<pid>,<dcs>,<s New message is received and output
| 15  |     |     |     | See AT+CNMI  |
| --- | --- | --- | --- | ------------ |
ca>,<tosca>,<length>]<CR>< directly to TE (Text mode)
LF><data>
+CBM: <length><CR><LF><p New  CBM  is  received  and  output
| 16  |     |     |     | See AT+CNMI  |
| --- | --- | --- | --- | ------------ |
du>  directly (PDU mode)
+CBM: <sn>,<mid>,<dcs>,<p
|                              | New  CBM  | is  received  | and  output  |              |
| ---------------------------- | --------- | ------------- | ------------ | ------------ |
| 17  age>,<pages><CR><LF><dat |           |               |              | See AT+CNMI  |
directly to TE (Text mode)
a>
+CDS: <length><CR><LF><p New  CDS  is  received  and  output
| 18  |     |     |     | See AT+CNMI  |
| --- | --- | --- | --- | ------------ |
du>  directly (PDU mode)
+CDS: <fo>,<mr>,[<ra>],[<tor New  CDS  is  received  and  output
| 19  |     |     |     | See AT+CNMI  |
| --- | --- | --- | --- | ------------ |
a>],<scts>,<dt>,<st>  directly to TE (Text mode)
|                           | New  message  | status  | report  | is           |
| ------------------------- | ------------- | ------- | ------- | ------------ |
| 20  +CDSI: <mem>,<index>  |               |         |         | See AT+CNMI  |
received and saved to memory
+COLP: <number>,<type>,[<
COL (connected line) presentation at
| 21  subaddr>],[<satype>],[<alph |     |     |     | AT+COLP=1  |
| ------------------------------- | --- | --- | --- | ---------- |
TE for a mobile originated call
a>]
+CLIP: <number>,<type>,[su
22  baddr],[satype],[<alpha>],<C Mobile terminating call indication  AT+CLIP=1
LI validity>
|                     | An  incoming  | call  is  indicated  | to  | TE        |
| ------------------- | ------------- | -------------------- | --- | --------- |
| 23  +CRING: <type>  |               |                      |     | AT+CRC=1  |
with URC instead of normal RING
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                267 / 282

                                          5G Module Series
+CCWA: <number>,<type>,<
class>[,<alpha>][,<CLI_validi
| 24  | Call waiting indication  |     |     | AT+CCWA=1,1  |
| --- | ------------------------ | --- | --- | ------------ |
ty>[,<subaddr>,<satype>[,<p
riority>]]]
+CSSI intermediate result code
| 25  +CSSI: <code1>  |     |     |     | AT+CSSN=1  |
| ------------------- | --- | --- | --- | ---------- |
presentation status to TE
|                     | +CSSU  URC  | presentation  | status  | to             |
| ------------------- | ----------- | ------------- | ------- | -------------- |
| 26  +CSSU: <code2>  |             |               |         | AT+CSSN=<n>,1  |
TE
| 27  RDY  | MT initialization is successful  |     |     | -   |
| -------- | -------------------------------- | --- | --- | --- |
-
| 28  +CFUN: 1  | All MT functions are available  |     |     |     |
| ------------- | ------------------------------- | --- | --- | --- |
-
| 29  +CPIN: <state>  | (U)SIM card pin state  |     |     |     |
| ------------------- | ---------------------- | --- | --- | --- |
-
| 30  +QIND: SMS DONE  | SMS initialization finished  |     |     |     |
| -------------------- | ---------------------------- | --- | --- | --- |
-
| 31  +QIND: PB DONE  | Phonebook initialization finished  |     |     |     |
| ------------------- | ---------------------------------- | --- | --- | --- |
-
| 32  +CPIN: NOT READY  | (U)SIM card is not ready  |     |     |           |
| --------------------- | ------------------------- | --- | --- | --------- |
| 33  POWERED DOWN      | Module power down         |     |     | AT+QPOWD  |
+CGEV: REJECT <PDP_typ A network request for PDP activation,
| 34  |     |     |     | AT+CGEREP=2,1  |
| --- | --- | --- | --- | -------------- |
e>,<PDP_addr>  and automatically rejected.
+CGEV: NW REACT <PDP_ty
| 35  | Network request PDP reactivation  |     |     | AT+CGEREP=2,1  |
| --- | --------------------------------- | --- | --- | -------------- |
pe>,<PDP_addr>,[<cid>]
+CGEV: NW DEACT <PDP_ty
| 36  | Network-forced context deactivation  |     |     | AT+CGEREP=2,1  |
| --- | ------------------------------------ | --- | --- | -------------- |
pe>,<PDP_addr>,[<cid>]
+CGEV: ME DEACT <PDP_ty
| 37  | ME-forced context deactivation.  |     |     | AT+CGEREP=2,1  |
| --- | -------------------------------- | --- | --- | -------------- |
pe>,<PDP_addr>,[<cid>]
38  +CGEV: NW DETACH  Network-forced packet domain detach.  AT+CGEREP=2,1
|                       | Mobile  equipment-forced  |     | packet  |                |
| --------------------- | ------------------------- | --- | ------- | -------------- |
| 39  +CGEV: ME DETACH  |                           |     |         | AT+CGEREP=2,1  |
domain detach.
40  +CGEV: NW CLASS <class>  Network-forced change of MS class.  AT+CGEREP=2,1
|                              | Mobile  equipment-forced  |     | change  | of             |
| ---------------------------- | ------------------------- | --- | ------- | -------------- |
| 41  +CGEV: ME CLASS <class>  |                           |     |         | AT+CGEREP=2,1  |
MS class.
| 42  +CGEV: PDN ACT<cid>  | Context activated.  |     |     | AT+CGEREP=2,1  |
| ------------------------ | ------------------- | --- | --- | -------------- |
43  +CGEV: PDN DEACT<cid>  Context deactivated.  AT+CGEREP=2,1
|     | Signal strength and channel bit error  |     |     | AT+QINDCFG="cs |
| --- | -------------------------------------- | --- | --- | -------------- |
44  +QIND: "csq",<rssi>,<ber>
|     | rate changed.  |     |     | q",1  |
| --- | -------------- | --- | --- | ----- |
AT+QINDCFG="s
| 45  +QIND: "smsfull",<storage>  | SMS storage is full.  |     |     |     |
| ------------------------------- | --------------------- | --- | --- | --- |
msfull",1
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                268 / 282

                                          5G Module Series
AT+QINDCFG="rin
| 46  RING  |     | Incoming call.  |     |     |     |
| --------- | --- | --------------- | --- | --- | --- |
g",1
AT+QINDCFG="ac
47  +QIND: "act",<actvalue>  Network access technology changed.
t",1
^DSCI: <id>,<dir>,<stat>,<ty
| 48  |     | Call status indication.  |     |     | AT^DSCI=1  |
| --- | --- | ------------------------ | --- | --- | ---------- |
pe>,<number>,<num_type>
+CLIP: <number>,<type>,[su
|                                  |     | Calling  line  | identity  (CLI)  | of  calling  |            |
| -------------------------------- | --- | -------------- | ---------------- | ------------ | ---------- |
| 49  baddr],[satype],[<alpha>],<C |     |                |                  |              | AT+CLIP=1  |
party of a mobile terminated call
LI_validity>
+CUSD: <status>[,<rspstr>, USSD  response  from  network,  or  a
| 50  |     |     |     |     | AT+CUSD=1  |
| --- | --- | --- | --- | --- | ---------- |
[<dcs>]]  network initiated operation.
| 52  +CR: <serv>  |     | Service reporting control.  |     |     | See AT+CR  |
| ---------------- | --- | --------------------------- | --- | --- | ---------- |

13.8. SMS Character Sets Conversions

In 3GPP TS 23.038 Data Coding Scheme (DCS), 3GPP defines three kinds of alphabets in SMS: GSM
7-bit  default,  8-bit  data,  and  UCS2  (16-bit)  alphabets.  AT+CSMP  can  set  the  DCS  in  text  mode
(AT+CMGF=1). In text mode, DCS (Data Coding Scheme) and AT+CSCS determine the way of SMS
text input or output.

Table 12: SMS Text Input or Output Methods
| DCS        | AT+CSCS  | SMS Text Input or Output Methods     |     |     |     |
| ---------- | -------- | ------------------------------------ | --- | --- | --- |
| GSM 7-bit  | GSM      | Input or output GSM character sets.  |     |     |     |
Input or output IRA character sets.
GSM 7-bit  IRA  Input: UE will convert IRA characters to GSM characters.
Output: UE will convert GSM characters to IRA characters.
Input or output a hex string similar to PDU mode. Only characters 0-9
and A-F supported.
| GSM 7-bit  | UCS2  |     |     |     |     |
| ---------- | ----- | --- | --- | --- | --- |
Input: UE converts UCS2 hex string to GSM characters.
Output: UE converts GSM characters to UCS2 hex string.
Ignore the value of AT+CSCS, input or output a hex string similar to
| UCS2  | -   |     |     |     |     |
| ----- | --- | --- | --- | --- | --- |
PDU mode. Only characters 0-9 and A-F supported.
Ignore the value of AT+CSCS, input or output a hex string similar to
| 8-bit  | -   |     |     |     |     |
| ------ | --- | --- | --- | --- | --- |
PDU mode. Only characters 0-9 and A-F supported.

When DCS = GSM 7-bit, the input or output needs conversion. The detailed conversion tables are shown
below.
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                269 / 282

                                          5G Module Series
Table 13: Input Conversion Table (DCS=GSM 7-bit and AT+CSCS="GSM")
| No.  | 0   | 1        | 2   | 3   | 4   | 5   | 6   | 7   |
| ---- | --- | -------- | --- | --- | --- | --- | --- | --- |
| 0    | 00  | 10       | 20  | 30  | 40  | 50  | 60  | 70  |
| 1    | 01  | 11       | 21  | 31  | 41  | 51  | 61  | 71  |
| 2    | 02  | 12       | 22  | 32  | 42  | 52  | 62  | 72  |
| 3    | 03  | 13       | 23  | 33  | 43  | 53  | 63  | 73  |
| 4    | 04  | 14       | 24  | 34  | 44  | 54  | 64  | 74  |
| 5    | 05  | 15       | 25  | 35  | 45  | 55  | 65  | 75  |
| 6    | 06  | 16       | 26  | 36  | 46  | 56  | 66  | 76  |
| 7    | 07  | 17       | 27  | 37  | 47  | 57  | 67  | 77  |
| 8    | 08  | 18       | 28  | 38  | 48  | 58  | 68  | 78  |
| 9    | 09  | 19       | 29  | 39  | 49  | 59  | 69  | 79  |
| A    | 0A  | Submit   | 2A  | 3A  | 4A  | 5A  | 6A  | 7A  |
| B    | 0B  | Cancel   | 2B  | 3B  | 4B  | 5B  | 6B  | 7B  |
| C    | 0C  | 1C       | 2C  | 3C  | 4C  | 5C  | 6C  | 7C  |
| D    | 0D  | 1A       | 2D  | 3D  | 4D  | 5D  | 6D  | 7D  |
| E    | 0E  | 1E       | 2E  | 3E  | 4E  | 5E  | 6E  | 7E  |
| F    | 0F  | 1F       | 2F  | 3F  | 4F  | 5F  | 6F  | 7F  |

Table 14: Output Conversion Table (DCS=GSM 7-bit and AT+CSCS="GSM")
| No.  | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0    | 00  | 10  | 20  | 30  | 40  | 50  | 60  | 70  |
| 1    | 01  | 11  | 21  | 31  | 41  | 51  | 61  | 71  |
| 2    | 02  | 12  | 22  | 32  | 42  | 52  | 62  | 72  |
| 3    | 03  | 13  | 23  | 33  | 43  | 53  | 63  | 73  |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                270 / 282

                                          5G Module Series
| 4   | 04    | 14  | 24  | 34  | 44  | 54  | 64  | 74  |
| --- | ----- | --- | --- | --- | --- | --- | --- | --- |
| 5   | 05    | 15  | 25  | 35  | 45  | 55  | 65  | 75  |
| 6   | 06    | 16  | 26  | 36  | 46  | 56  | 66  | 76  |
| 7   | 07    | 17  | 27  | 37  | 47  | 57  | 67  | 77  |
| 8   | 08    | 18  | 28  | 38  | 48  | 58  | 68  | 78  |
| 9   | 09    | 19  | 29  | 39  | 49  | 59  | 69  | 79  |
| A   | 0D0A  |     |     |     |     |     |     |     |
|     |       |     | 2A  | 3A  | 4A  | 5A  | 6A  | 7A  |
| B   | 0B    |     | 2B  | 3B  | 4B  | 5B  | 6B  | 7B  |
| C   | 0C    | 1C  | 2C  | 3C  | 4C  | 5C  | 6C  | 7C  |
| D   | 0D    | 1A  | 2D  | 3D  | 4D  | 5D  | 6D  | 7D  |
| E   | 0E    | 1E  | 2E  | 3E  | 4E  | 5E  | 6E  | 7E  |
| F   | 0F    | 1F  | 2F  | 3F  | 4F  | 5F  | 6F  | 7F  |

Table 15: GSM Extended Characters (GSM Encode)
| No.  | 0   | 1     | 2     | 3   | 4     | 5   | 6   | 7   |
| ---- | --- | ----- | ----- | --- | ----- | --- | --- | --- |
| 0    |     |       |       |     | 1B40  |     |     |     |
| 1    |     |       |       |     |       |     |     |     |
| 2    |     |       |       |     |       |     |     |     |
| 3    |     |       |       |     |       |     |     |     |
| 4    |     | 1B14  |       |     |       |     |     |     |
| 5    |     |       |       |     |       |     |     |     |
| 6    |     |       |       |     |       |     |     |     |
| 7    |     |       |       |     |       |     |     |     |
| 8    |     |       | 1B28  |     |       |     |     |     |
| 9    |     |       | 1B29  |     |       |     |     |     |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                271 / 282

                                          5G Module Series
| A   |     |     |       |       |     |     |     |     |
| --- | --- | --- | ----- | ----- | --- | --- | --- | --- |
| B   |     |     |       |       |     |     |     |     |
| C   |     |     |       | 1B3C  |     |     |     |     |
| D   |     |     |       | 1B3D  |     |     |     |     |
| E   |     |     |       | 1B3E  |     |     |     |     |
| F   |     |     | 1B2F  |       |     |     |     |     |

Table 16: Input Conversion Table (DCS = GSM 7-bit and AT+CSCS="IRA")
| No.  | 0          | 1        | 2   | 3   | 4   | 5     | 6   | 7     |
| ---- | ---------- | -------- | --- | --- | --- | ----- | --- | ----- |
| 0    |            | 20       | 20  | 30  | 00  | 50    | 20  | 70    |
| 1    | 20         | 20       | 21  | 31  | 41  | 51    | 61  | 71    |
| 2    | 20         | 20       | 22  | 32  | 42  | 52    | 62  | 72    |
| 3    | 20         | 20       | 23  | 33  | 43  | 53    | 63  | 73    |
| 4    | 20         | 20       | 02  | 34  | 44  | 54    | 64  | 74    |
| 5    | 20         | 20       | 25  | 35  | 45  | 55    | 65  | 75    |
| 6    | 20         | 20       | 26  | 36  | 46  | 56    | 66  | 76    |
| 7    | 20         | 20       | 27  | 37  | 47  | 57    | 67  | 77    |
| 8    | Backspace  | 20       | 28  | 38  | 48  | 58    | 68  | 78    |
| 9    | 20         | 20       | 29  | 39  | 49  | 59    | 69  | 79    |
| A    | 0A         | Submit   | 2A  | 3A  | 4A  | 5A    | 6A  | 7A    |
| B    | 20         | Cancel   | 2B  | 3B  | 4B  | 1B3C  | 6B  | 1B28  |
| C    | 20         | 20       | 2C  | 3C  | 4C  | 1B2F  | 6C  | 1B40  |
| D    | 0D         | 20       | 2D  | 3D  | 4D  | 1B3E  | 6D  | 1B29  |
| E    | 20         | 20       | 2E  | 3E  | 4E  | 1B14  | 6E  | 1B3D  |
| F    | 20         | 20       | 2F  | 3F  | 4F  | 11    | 6F  | 20    |

RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                272 / 282

                                          5G Module Series
Table 17: IRA Extended Characters
| No.  | A   | B   |     | C   | D   | E   |     | F   |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- |
|      |     |     |     |     | 20  | 7F  |     | 20  |
| 0    | 20  | 20  |     | 20  |     |     |     |     |
| 1    | 40  | 20  |     | 20  | 5D  | 20  |     | 7D  |
|      | 20  |     |     |     |     |     |     | 08  |
| 2    |     | 20  |     | 20  | 20  | 20  |     |     |
| 3    | 01  | 20  |     | 20  | 20  | 20  |     | 20  |
|      | 24  |     |     | 5B  |     | 7B  |     |     |
| 4    |     | 20  |     |     | 20  |     |     | 20  |
|      | 03  |     |     | 0E  |     | 0F  |     |     |
| 5    |     | 20  |     |     | 20  |     |     | 20  |
| 6    | 20  | 20  |     | 1C  | 5C  | 1D  |     | 7C  |
|      | 5F  |     |     | 09  | 20  | 20  |     | 20  |
| 7    |     | 20  |     |     |     |     |     |     |
| 8    | 20  | 20  |     | 20  | 0B  | 04  |     | 0C  |
|      |     |     |     | 1F  |     | 05  |     | 06  |
| 9    | 20  | 20  |     |     | 20  |     |     |     |
| A    | 20  | 20  |     | 20  | 20  | 20  |     | 20  |
| B    | 20  | 20  |     | 20  | 20  | 20  |     | 20  |
| C    | 20  | 20  |     | 20  | 5E  | 07  |     | 7E  |
| D    | 20  | 20  |     | 20  | 20  | 20  |     | 20  |
| E    | 20  | 20  |     | 20  | 20  | 20  |     | 20  |
|      |     | 60  |     |     | 1E  |     |     |     |
| F    | 20  |     |     | 20  |     | 20  |     | 20  |

Table 18: Output Conversion Table (DCS = GSM 7-bit and AT+CSCS="IRA")
| No.  | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0    | 40  | 20  | 20  | 30  | A1  | 50  | BF  | 70  |
| 1    | A3  | 5F  | 21  | 31  | 41  | 51  | 61  | 71  |
| 2    | 24  | 20  | 22  | 32  | 42  | 52  | 62  | 72  |
| 3    | A5  | 20  | 23  | 33  | 43  | 53  | 63  | 73  |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                273 / 282

                                          5G Module Series
| 4   | E8    | 20  | A4  | 34  | 44  | 54  | 64  | 74  |
| --- | ----- | --- | --- | --- | --- | --- | --- | --- |
| 5   | E9    | 20  | 25  | 35  | 45  | 55  | 65  | 75  |
| 6   | F9    | 20  | 26  | 36  | 46  | 56  | 66  | 76  |
| 7   | EC    | 20  | 27  | 37  | 47  | 57  | 67  | 77  |
| 8   | F2    | 20  | 28  | 38  | 48  | 58  | 68  | 78  |
| 9   | C7    | 20  | 29  | 39  | 49  | 59  | 69  | 79  |
| A   | 0D0A  |     | 2A  | 3A  | 4A  | 5A  | 6A  | 7A  |
| B   | D8    |     | 2B  | 3B  | 4B  | C4  | 6B  | E4  |
| C   | F8    | C6  | 2C  | 3C  | 4C  | D6  | 6C  | F6  |
| D   | 0D    | E6  | 2D  | 3D  | 4D  | D1  | 6D  | F1  |
| E   | C5    | DF  | 2E  | 3E  | 4E  | DC  | 6E  | FC  |
| F   | E5    | C9  | 2F  | 3F  | 4F  | A7  | 6F  | E0  |

Table 19: GSM Extended Characters (ISO-8859-1/Unicode)
| No.  | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0    |     |     |     |     | 7C  |     |     |     |
| 1    |     |     |     |     |     |     |     |     |
| 2    |     |     |     |     |     |     |     |     |
| 3    |     |     |     |     |     |     |     |     |
| 4    |     | 5E  |     |     |     |     |     |     |
| 5    |     |     |     |     |     |     |     |     |
| 6    |     |     |     |     |     |     |     |     |
| 7    |     |     |     |     |     |     |     |     |
| 8    |     |     | 7B  |     |     |     |     |     |
| 9    |     |     | 7D  |     |     |     |     |     |
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual                                274 / 282

5G Module Series
A
B
C 5B
D 7E
E 5D
F 5C
Because the low 8-bit of UCS2 character is the same as the IRA character:
⚫ The conversion table of DCS = GSM 7-bit and AT+CSCS="UCS2" is similar to AT+CSCS="IRA".
⚫ The conversion table of DCS= GSM 7-bit and AT+CSCS="GSM" is similar to AT+CSCS="GSM".
⚫ The conversion table of DCS = GSM 7-bit and AT+CSCS="IRA" is similar to AT+CSCS="IRA".
⚫ The conversion table of DCS = GSM 7-bit and AT+CSCS="UCS2" is similar to AT+CSCS="IRA".
The method of SMS text input or output is different. See Table 13 for more details.
13.9. Release Cause Text List of AT+CEER
Table 20: Release Cause Text List of AT+CEER
CS Internal Cause
No cause information available (default)
Phone is offline
No service available
Network release, no reason given
Received incoming call
Client ended call
UIM not present
Access attempt already in progress
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 275 / 282

5G Module Series
Access failure, unknown source
Concur service not supported by network
No response received from network
GPS call ended for user call
SMS call ended for user call
Data call ended for emergency call
Rejected during redirect or handoff
Lower-layer ended call
Call origination request failed
Client rejected incoming call
Client rejected setup indication
Network ended call
No funds available
No service available
Full service not available
Maximum packet calls exceeded
Video connection lost
Video protocol closed after setup
Video protocol setup failure
Internal error
CS Network Cause
Unassigned/unallocated number
No route to destination
Channel unacceptable
Operator determined barring
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 276 / 282

5G Module Series
Normal call clearing
User busy
No user responding
User alerting, no answer
Call rejected
Number changed
Non selected user clearing
Destination out of order
Invalid/incomplete number
Facility rejected
Response to status enquiry
Normal, unspecified
No circuit/channel available
Network out of order
Temporary failure
Switching equipment congestion
Access information discarded
Requested circuit/channel not available
Resources unavailable, unspecified
Quality of service unavailable
Requested facility not subscribed
Incoming calls barred within the CUG
Bearer capability not authorized
Bearer capability not available
Service/option not available
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 277 / 282

5G Module Series
Bearer service not implemented
ACM >= ACM max
Requested facility not implemented
Only RDI bearer is available
Service/option not implemented
Invalid transaction identifier value
User not CUG member
Incompatible destination
Invalid transit network selection
Semantically incorrect message
Invalid mandatory information
Message non-existent/not implemented
Message type not compatible with state
IE non-existent/not implemented
Conditional IE error
Message not compatible with state
Recovery on timer expiry
Protocol error, unspecified
Interworking, unspecified
CS Network Reject
IMSI unknown in HLR
Illegal MS
IMSI unknown in VLR
IMEI not accepted
Illegal ME
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 278 / 282

5G Module Series
GPRS services not allowed
GPRS and non GPRS services not allowed
MS identity cannot be derived
Implicitly detached
PLMN not allowed
Location area not allowed
Roaming not allowed
GPRS services not allowed in PLMN
No suitable cells in location area
MSC temporary not reachable
Network failure
MAC failure
Synch failure
Congestion
GSM authentication unacceptable
Service option not supported
Requested service option not subscribed
Service option temporary out of order
Call cannot be identified
No PDP context activated
Semantically incorrect message
Invalid mandatory information
Message type non-existent
Message type not compatible with state
Information element non-existent
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 279 / 282

5G Module Series
Message not compatible with state
RR release indication
RR random access failure
RRC release indication
RRC close session indication
RRC open session failure
Low level failure
Low level failure, no redial allowed
Invalid SIM
No service
Timer T3230 expired
No cell available
Wrong state
Access class blocked
Abort message received
Other cause
Timer T303 expired
No resources
Release pending
Invalid user data
PS Internal Cause
Invalid connection identifier
Invalid NSAPI
Invalid primary NSAPI
PDP establish timeout
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 280 / 282

5G Module Series
Invalid field
SNDCP failure
RAB setup failure
No GPRS context
PDP activate timeout
PDP modify timeout
PDP inactive max timeout
PDP lower layer error
PDP duplicate
Access technology change
PDP unknown reason
CS PS Network Cause
LLC or SNDCP failure
Insufficient resources
Missing or unknown APN
Unknown PDP address or PDP type
User authentication failed
Activation rejected by GGSN
Activation rejected, unspecified
Service option not supported
Requested service option not subscribed
Service option temporary out of order
NSAPI already used (not sent)
Regular deactivation
QoS not accepted
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 281 / 282

5G Module Series
Network failure
Reactivation required
Feature not supported
Semantic error in the TFT operation
Syntactical error in the TFT operation
Unknown PDP context
PDP context without TFT already activated
Semantic errors in packet filter
Syntactical errors in packet filter
Invalid transaction identifier
Semantically incorrect message
Invalid mandatory information
Message non-existent/not implemented
Message type not compatible with state
IE non-existent/not implemented
Conditional IE error
Message not compatible with state
Protocol error, unspecified
RG520N&RG525F&RG5x0F&RM5x0N_Series_AT_Commands_Manual 282 / 282