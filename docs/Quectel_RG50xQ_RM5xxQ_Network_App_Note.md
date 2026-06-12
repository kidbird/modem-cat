RG50xQ&RM5xxQ Series
Network Application Note
5G Module Series
Version: 1.0.0
Date: 2023-10-19
Status: Preliminary

5G Module Series
At Quectel, our aim is to provide timely and comprehensive services to our customers. If you
require any assistance, please contact our headquarters:
Quectel Wireless Solutions Co., Ltd.
Building 5, Shanghai Business Park Phase III (Area B), No.1016 Tianlin Road, Minhang District, Shanghai
200233, China
Tel: +86 21 5108 6236
Email: info@quectel.com
Or our local offices. For more information, please visit:
http://www.quectel.com/support/sales.htm.
For technical support, or to report documentation errors, pleasel visit:
e
http://www.quectel.com/support/technical.htm.
Or email us at: support@quectel.com.
t
c
l
a
Legal Notices
e
i
We offer information as a service to you. The provided information is based on your requirements and we
u t
make every effort to ensure its quality. You agree that you are responnsible for using independent analysis
and evaluatioQn in designing intended products, and we provide reference designs for illustrative purposes
only. Before using any hardware, software or service guideed by this document, please read this notice
carefully. Even though we employ commercially reasonable efforts to provide the best possible experience,
d
you hereby acknowledge and agree that this document and related services hereunder are provided to
you on an “as available” basis. We may reviise or restate this document from time to time at our sole
f
discretion without any prior notice to you.
n
Use and Disoclosure Restrictions
C
License Agreements
Documents and information provided by us shall be kept confidential, unless specific permission is granted.
They shall not be accessed or used for any purpose except as expressly provided herein.
Copyright
Our and third-party products hereunder may contain copyrighted material. Such copyrighted material shall
not be copied, reproduced, distributed, merged, published, translated, or modified without prior written
consent. We and the third party have exclusive rights over copyrighted material. No license shall be
granted or conveyed under any patents, copyrights, trademarks, or service mark rights. To avoid
ambiguities, purchasing in any form cannot be deemed as granting a license other than the normal non-
exclusive, royalty-free license to use the material. We reserve the right to take legal action for
noncompliance with abovementioned requirements, unauthorized use, or other illegal or malicious use of
the material.
RG50xQ&RM5xxQ_Series_Network_Application_Note 1 / 136

5G Module Series
Trademarks
Except as otherwise set forth herein, nothing in this document shall be construed as conferring any rights
to use any trademark, trade name or name, abbreviation, or counterfeit product thereof owned by Quectel
or any third party in advertising, publicity, or other aspects.
Third-Party Rights
This document may refer to hardware, software and/or documentation owned by one or more third parties
(“third-party materials”). Use of such third-party materials shall be governed by all restrictions and
obligations applicable thereto.
We make no warranty or representation, either express or implied,l regarding the third-party materials,
e
including but not limited to any implied or statutory, warranties of merchantability or fitness for a particular
purpose, quiet enjoyment, system integration, information accuracy, and non-infringement of any third-
t
party intellectual property rights with regard cto the licensed technology or use thereof. Nothing herein
l
constitutes a representation or warranty by us to either develop, enhance, modify, distriabute, market, sell,
e
offer for sale, or otherwise maintain production of any our products or any other hardware, software, device,
i
tool, information, or product. We moreover disclaim any and all warranties arising from the course of
u t
dealing or usage of trade. n
Q
e
Privacy Policy
d
To implement module functionality, certain device data are uploaded to Quectel’s or third-party’s servers,
including carriers, chipset suppliers or custoimer-designated servers. Quectel, strictly abiding by the
f
relevant laws and regulations, shall retain, use, disclose or otherwise process relevant data for the purpose
n
of performing the service only or as permitted by applicable laws. Before data interaction with third parties,
please be informed of their privacy and data security policy.
o
C
Disclaimer
a) We acknowledge no liability for any injury or damage arising from the reliance upon the information.
b) We shall bear no liability resulting from any inaccuracies or omissions, or from the use of the
information contained herein.
c) While we have made every effort to ensure that the functions and features under development are
free from errors, it is possible that they could contain errors, inaccuracies, and omissions. Unless
otherwise provided by valid agreement, we make no warranties of any kind, either implied or express,
and exclude all liability for any loss or damage suffered in connection with the use of features and
functions under development, to the maximum extent permitted by law, regardless of whether such
loss or damage may have been foreseeable.
d) We are not responsible for the accessibility, safety, accuracy, availability, legality, or completeness of
information, advertising, commercial offers, products, services, and materials on third-party websites
and third-party resources.
Copyright © Quectel Wireless Solutions Co., Ltd. 2023. All rights reserved.
RG50xQ&RM5xxQ_Series_Network_Application_Note 2 / 136

5G Module Series
About the Document
Revision History
l
e
Version Date Author Description
Amos ZHANG/ t
- 2020-12-20 Spawn ZHANG/ cCreation of the document l
Yaalon PAN a
e
Franco WANG/
i
Supawn ZHANG/ t
Allen YU/ n
Q
Tang JIE/
e
Charles WANG/
1.0.0 2023-10-19 Preliminary
Miracle MA/
d
Kwan Yi Chiet/
Aiman Farhan/ i
f
Aydan DING/
n
Louis YIN
o
C
RG50xQ&RM5xxQ_Series_Network_Application_Note 3 / 136

5G Module Series
Contents
About the Document .................................................................................................................................. 3
Contents ...................................................................................................................................................... 4
Table Index .................................................................................................................................................. 7
1 Introduction ......................................................................................................................................... 8
2 AT Command Introduction ................................................................................................................ 9
2.1. Definitions ............................................................................................................................... 9
2.2. AT Command Syntax ............................................................................................................. 9
2.3. Declaration of AT Command Examples ................................ ............................................... 10
l
3 Network Service Command ................................................................................................................ 11
e
3.1. Network Register Status ....................................................................................................... 11
3.1.1. AT+COPS Operator Selection ......t............................................................................. 11
3.1.2. AT+CREG Network Regisctration Status ...........................................................
l
......... 13
3.1.3. AT+CGREG Network Registration Status ..........................................a........................ 15
e
3.1.4. AT+CEREG EPS Network Registration Status .......................................................... 17
i
3.1.5. AT+C5GuREG 5GS Network Registration Status .................t...................................... 18
3.2. Network Paramenter Status ............................................n..................................................... 20
Q
3.2.1. AT+QNWINFO Query Network Information ............................................................... 20
e
3.2.2. AT+QENG Query Primary Serving Cell and Neighbor Cell Information .................... 22
3.2.3. AT+QCAINFO Query Carrier Aggregation Parameters ............................................. 27
d
3.2.4. AT+QENDC Query EN-DC Status ............................................................................. 31
3.2.5. AT+QSCAN Search Nearbiy Cells .............................................................................. 32
f
3.3. Network Signal Strength ....................................................................................................... 36
n
3.3.1. AT+CSQ Signal Quality Report .................................................................................. 36
3.3.2. AT+oQCSQ Report Signal Quality ............................................................................... 38
3.3.3. AT+QRSRP Report RSRP ......................................................................................... 40
C
3.3.4. AT+QRSRQ Report RSRQ ........................................................................................ 41
3.3.5. AT+QSINR Report SINR ............................................................................................ 42
3.3.6. AT+QRSSI Report RSSI ............................................................................................ 42
3.4. General Commands ............................................................................................................. 44
3.4.1. AT+CPOL Preferred Operator List ............................................................................. 44
3.4.2. AT+CPLS Select PLMN Selector ............................................................................... 45
3.4.3. AT+CGDCONT Define PDP Contexts ....................................................................... 46
3.4.4. AT+COPN Read Operator Names ............................................................................. 48
3.4.5. AT+CTZU Automatic Time Zone Update ................................................................... 49
3.4.6. AT+CTZR Time Zone Reporting ................................................................................ 50
3.4.7. AT+CCLK Clock ......................................................................................................... 52
3.4.8. AT+QLTS Obtain the Latest Time Synchronized through Network ........................... 52
3.4.9. AT+QSPN Query Service Provider Name ................................................................. 54
3.4.10. AT+QNETRC Get the Net Reject Cause ................................................................... 55
3.5. Packet Domain Commands.................................................................................................. 59
RG50xQ&RM5xxQ_Series_Network_Application_Note 4 / 136

5G Module Series
3.5.1. AT+CGACT Activate/Deactive PDP Contexts ........................................................... 59
3.5.2. AT+CGATT Attachment or Detachment of PS........................................................... 61
3.5.3. AT+CGPADDR Show PDP Addresses ...................................................................... 62
3.5.4. AT+CGEQOSRDP Read EPS Quality of Service Dynamic Parameters ................... 63
3.5.5. AT+CGTFTRDP Read Traffic Flow Template Dynamic Parameters ........................ 64
3.5.6. AT+QGPAPN Query Activated APNs ........................................................................ 66
3.5.7. AT+QWDSCFG Wireless Device Service .................................................................. 67
3.5.7.1. AT+QWDSCFG="lte_attach_pdn" Set LTE Attachment PDN ..................... 68
3.5.7.2. AT+QWDSCFG="operator_reserved_pco" Set Operator Reserved PCO .. 69
3.6. AT+QNWLOCK Network Cell Lock ................................................................................... 70
3.6.1. AT+QNWLOCK="common/4g" Lock Module to the Specified 4G Cell ...................... 70
3.6.2. AT+QNWLOCK="common/5g" Lock Module to the Sp ecified 5G Cell ...................... 71
3.6.3. AT+QNWLOCK="save_ctrl" Configure Whether tol Save the Locked Cell ................ 73
e
3.6.4. AT+QNWLOCK="common/4g_ext" Lock Module to the Specified 4G Cell ............... 74
3.7. AT+QNWCFG Configure and Query Network Parameters ............................................... 75
t
3.7.1. AT+QNWCFG="lte_cell_id"c Read Cell ID Under LTE ............................................... 76
l
3.7.2. AT+QNWCFG="nr5g_cell_id" Read Cell ID Under NR5G SA ...........a........................ 77
e
3.7.3. AT+QNWCFG="up/down" Get Average Uplink Rate and Downlink Rate in Delta Time
i
..............
u
...................................................................................
t
...................................... 77
3.7.4. AT+QNWCFG="dss_enable" Enable/Disable DSSn Function .................................... 78
3.7.Q5. AT+QNWCFG="lte_dl_tx_mode" Query Downlink Transmission Mode .................... 79
3.7.6. AT+QNWCFG="clr_rplmn" Clear RPLMeN ................................................................. 80
3.7.7. AT+QNWCFG="dis_rplmnact" Enable/Disable RPLMNACT ..................................... 80
d
3.7.8. AT+QNWCFG="lte_ambr" Query LTE AMBR ........................................................... 81
3.7.9. AT+QNWCFG="nr5g_ambri" Query NR5G AMBR ..................................................... 82
f
3.7.10. AT+QNWCFG="dis_4mimo_enable" Control 4*MIMO of LTE Band ......................... 84
n
3.7.11. AT+QNWCFG="encryp_alg_support" Query Supported Encryption Algorithms ....... 84
3.7.12. AT+QNWCFG="integ_alg_support” Query Supported Integrity Algorithm ................ 86
o
3.7.13. AT+QNWCFG="data_roaming " Control Data Roaming ........................................... 87
C3.7.14. AT+QNWCFG="nr5g_earfcn_lock" Lock the NR5G EARFCN .................................. 88
3.7.15. AT+QNWCFG="lte_earfcn_lock" Lock the LTE EARFCN ......................................... 89
3.7.16. AT+QNWCFG="used_algo" Enable/Disable Encryption and Integrity Algorithm ...... 90
3.7.17. AT+QNWCFG="nr5g_pref_freq_list" Configure NR5G Preference Frequency ........ 92
3.7.18. AT+QNWCFG="lte_pref_freq_list" Configure LTE Preference Frequency ............... 93
3.7.19. AT+QNWCFG="ehplmn_config" Configure EHPLMN List ........................................ 94
3.7.20. AT+QNWCFG="rrc_state" Query RAT and RRC State ............................................. 95
3.7.21. AT+QNWCFG="lte_mimo_layers" Query LTE MIMO Layers .................................... 96
3.7.22. AT+QNWCFG="lte_band_priority" Set LTE Band Priority ......................................... 97
3.7.23. AT+QNWCFG="nr5g_band_priority" Set NR5G Band Priority .................................. 98
3.7.24. AT+QNWCFG="cause7_map_cause14" Enable/Disable to Map cause7 to cause14 ..
....................................................................................................................................... 99
3.7.25. AT+QNWCFG="nr5g_ul_256qam" Enable/Disable NR5G UL 256QAM ................. 100
3.7.26. AT+QNWCFG="thin_ui_cfg" Configure Default Operating Mode After Power-up .. 101
3.7.27. AT+QNWCFG="lte_pco" Query LTE PCO Information ........................................... 102
RG50xQ&RM5xxQ_Series_Network_Application_Note 5 / 136

5G Module Series
3.7.28. AT+QNWCFG="msisdn" Query MSISDN From the Network .................................. 104
3.7.29. AT+QNWCFG="lte_fgi_fdd" Configure LTE FGI for FDD Bands ............................ 106
3.7.30. AT+QNWCFG="lte_fgi_tdd" Configure LTE FGI for TDD Bands ............................ 108
3.7.31. AT+QNWCFG="sysmode" Query System Mode and Sub-mode ............................ 109
3.7.32. AT+QNWCFG="nitz_ons" Query PLMN Name from NITZ ...................................... 111
3.7.33. AT+QNWCFG="clr_guti" Clear GUTI ....................................................................... 111
3.8. AT+QNWPREFCFG Configure Network Searching Preferences ................................... 112
3.8.1. AT+QNWPREFCFG="gw_band" WCDMA Band Configuration .............................. 113
3.8.2. AT+QNWPREFCFG="lte_band" LTE Band Configuration ...................................... 114
3.8.3. AT+QNWPREFCFG="nsa_nr5g_band" NR5G NSA Band Configuration ............... 115
3.8.4. AT+QNWPREFCFG="nr5g_band" NR5G SA Band Configuration ......................... 116
3.8.5. AT+QNWPREFCFG="mode_pref" Network Search M ode Configuration ............... 117
3.8.6. AT+QNWPREFCFG="srv_domain" Service Domalin Configuration ........................ 118
e
3.8.7. AT+QNWPREFCFG="voice_domain" Voice Domain Configuration ....................... 119
3.8.8. AT+QNWPREFCFG="roam_pref" Roaming Preference Configuration .................. 120
t
3.8.9. AT+QNWPREFCFG="ue_ucsage_setting" UE Usage Setting Configuration ........... 121
l
3.8.10. AT+QNWPREFCFG="policy_band" Read Carrier Policy Band.........a...................... 122
e
3.8.11. AT+QNWPREFCFG="ue_capability_band" Query UE Band Capability ................. 123
i
3.8.12. AT+QNW
u
PREFCFG="rat_acq_order" Configure RAT Prior
t
ity ................................ 124
3.8.13. AT+QNWPREFCFG="nr5g_disable_mode" Disabnle NR5G .................................... 125
3.8.Q14. AT+QNWPREFCFG="rf_band " Query RF Bands Supported by Module ............... 126
3.8.15. AT+QNWPREFCFG="restore_band" Reestore to Default Bands Supported by Module
..................................................................................................................................... 128
d
3.9. Network Slice Command .................................................................................................... 128
3.9.1. AT+C5GNSSAI 5GS NSSiAI Setting ........................................................................ 128
f
3.9.2. AT+C5GNSSAIRDP Read 5GS NSSAI Dynamic Parameters ................................ 129
n
4 Summary of Error Codes ............................................................................................................... 133
o
5 Appendix Terms and Abbreviations ............................................................................................. 136
C
RG50xQ&RM5xxQ_Series_Network_Application_Note 6 / 136

5G Module Series
Table Index
Table 1: Applicable Modules ........................................................................................................................ 8
Table 2: Types of AT Commands ................................................................................................................ 9
Table 3: RAT State Corresponding to <RRC_state> ................................................................................. 96
Table 4: Combinations of Some FGIs ...................................................................................................... 107
Table 5: General Codes ........................................................................................................................... 133
Table 6: Terms and Abbreviations ........................................................................................................... 136
l
e
t
c
l
a
e
i
u t
n
Q
e
d
i
f
n
o
C
RG50xQ&RM5xxQ_Series_Network_Application_Note 7 / 136

5G Module Series
1
Introduction
The present document specifies a profile of AT commands and recommends that this profile be used for
controlling network services from a Terminal Equipment (TE) through Terminal Adaptor (TA). If there are
no special instructions, all AT commands in this article are applicable to all projects on the Quectel 5G
RG50xQ and RM5xxQ series modules.
l
e
Table 1: Applicable Modules
t
Module Series Model c l
a
e
RG500Q Series
i
u t
RG50xQ RG501Q-EU
n
Q
RG502Q Series
e
RM500Q Series
d
RM502Q-AE i
RM5xxQ f
RM50n5Q-AE
o
RM510Q-GL
C
RG50xQ&RM5xxQ_Series_Network_Application_Note 8 / 136

5G Module Series
2
AT Command Introduction
2.1. Definitions
⚫ <CR> Carriage return character.
l
⚫ <LF> Line feed character. e
⚫ <...> Parameter name. Angle brackets do not appear on the command line.
t
⚫ [...] Optional parameter of a command or an optional part of TA information response.
c
Square brackets do not appear on the command line. When an optional paralmeter is
a
not given in a coemmand, the new value equals to its previous value or the default
settings, unless otherwise specified. i
u t
⚫ Underline Default setting of a parameter.
n
Q
e
2.2. AT Command Syntax
d
i
All command lines must start with AT or fat and end with <CR>. Information responses and result codes
n
always start and end with a carriage return character and a line feed character:
<CR><LF><response><CR><LF>. In tables presenting commands and responses throughout this
o
document, only the commands and responses are presented, and <CR> and <LF> are deliberately omitted.
C
Table 2: Types of AT Commands
Command Type Syntax Description
Test the existence of corresponding Write
Test Command AT+<cmd>=? Command and return information about the
type, value, or range of its parameter.
Check the current parameter value of a
Read Command AT+<cmd>?
corresponding Write Command.
Write Command AT+<cmd>=<p1>[,<p2>[,<p3>[...]]] Set user-definable parameter value.
Return a specific information parameter or
Execution Command AT+<cmd>
perform a specific action.
RG50xQ&RM5xxQ_Series_Network_Application_Note 9 / 136

5G Module Series
2.3. Declaration of AT Command Examples
The AT command examples in this document are provided to help you learn about how to use the AT
commands introduced herein. The examples, however, should not be taken as Quectel’s recommendation
or suggestions about how you should design a program flow or what status you should set the module into.
Sometimes multiple examples may be provided for one AT command. However, this does not mean that
there exists a correlation among these examples and that they should be executed in a given sequence.
l
e
t
c
l
a
e
i
u t
n
Q
e
d
i
f
n
o
C
RG50xQ&RM5xxQ_Series_Network_Application_Note 10 / 136

5G Module Series
3
Network Service Command
3.1. Network Register Status
3.1.1. AT+COPS Operator Selection
l
e
This command returns the current operators and their status, and allows automatic or manual network
selection.
t
c
l
The Test Command returns a set of five parameters, each representing an operatora presenting in the
e
network. Any of the formats may be unavailable and should then be an empty field. The list of operators
i
shall be in the order of: homue network, networks referenced in (U)SIM and othe
t
r networks.
n
Q
The Read Command returns the current mode and the currently selected operator. If no operator is
e
selected, <format>, <oper> and <AcT> are omitted.
d
The Write Command forces an attempt to select and register the GSM/UMTS/EPS/5G network operator.
If the selected operator is not available, no iother operator shall be selected (except <mode>=4). The
f
format of selected operator name shall apply to further Read Commands (AT+COPS?).
n
AT+COPS Operator Selection
o
Test Command Response
C
AT+COPS=? +COPS: [list of supported (<stat>,long alphanumeric
<oper>,short alphanumeric <oper>,numeric
<oper>s[,<AcT>])s][,,(range of supported <mode>s),(range of
supported <format>s)]
OK
If there is any error related to MT functionality:
+CME ERROR: <err>
Read Command Response
AT+COPS? +COPS: <mode>[,<format>[,<oper>][,<AcT>]]
OK
If there is any error related to MT functionality:
RG50xQ&RM5xxQ_Series_Network_Application_Note 11 / 136

                                                                5G Module Series

+CME ERROR: <err>
Write Command  Response
AT+COPS=<mode>[,<format>[,<o OK
per>[,<AcT>]]]
If there is any error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time  180 s, determined by the network.
Characteristics  /
Reference
3GPP TS 27.007

l
e
Parameter
t
c
| <stat>    | Integer type. Availability of operators.  |     | l   |
| --------- | ----------------------------------------- | --- | --- |
a
|       | 0    Unknown  |     |     |
| ----- | ------------- | --- | --- |
e
|       | 1    Operator available  |     |     |
| ----- | ------------------------ | --- | --- |
i
|       | 2    Cuurrent operator   | t   |     |
| ----- | ------------------------ | --- | --- |
|       | 3    Operator forbidden  | n   |     |
Q
| <oper>    | String type. Operator in format as per <format>.  |     |     |
| --------- | ------------------------------------------------- | --- | --- |
e
| <mode>   | Integer type.  |     |     |
| -------- | -------------- | --- | --- |
      0    Automatic. Operator selecdtion (<oper> field is ignored).
      1    Manual operator selection (<oper> field shall be present and <AcT> optionally)
Manually deregister friom network
|       | 2    |     |     |
| ----- | ---- | --- | --- |
f
3  Set  only  <format>  (for  AT+COPS?  Read  Command),  and  do  not  attempt
n
registration/deregistration (<oper> and <AcT> fields are ignored). This value is
oinvalid in the response of Read Command.
4  Manual/automatic selection. <oper> field shall be presented. If manual selection
C
fails, automatic mode (<mode>=0) will be entered
| <format>  | Integer type.  |     |     |
| --------- | -------------- | --- | --- |
      0    Long format alphanumeric <oper> which can be up to 16 characters long
|       | 1    Short format alphanumeric <oper>  |     |     |
| ----- | -------------------------------------- | --- | --- |
      2    Numeric <oper>. GSM location area identification number
| <AcT>    | Integer type.  |     |     |
| -------- | -------------- | --- | --- |
  Access technology selected. Values 4, 5, 6 occur only in the response of Read Command
while MS is in data service state and is not intended for the AT+COPS Write Command.
|             | 2       UTRAN                        |     |     |
| ----------- | ------------------------------------ | --- | --- |
|             | 4       UTRAN W/HSDPA                |     |     |
|             | 5       UTRAN W/HSUPA                |     |     |
|             | 6       UTRAN W/HSDPA and HSUPA      |     |     |
|             | 7       E-UTRAN                      |     |     |
|             | 10      E-UTRAN connected to a 5GCN  |     |     |
|             | 11      NR connected to 5GCN         |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                12 / 136

5G Module Series
12 NG-RAN
13 E-UTRAN-NR dual connectivity
<err> Error codes. See Chapter 1 for details.
NOTE
When selecting NR5G SA network, <AcT> should be set to 12, and when registering NR5G SA network,
<AcT> returned by AT+COPS? is 11.
Example
AT+COPS=? //List all current networkl operators.
+COPS: (1,"CHN-UNICOM","UNICOM","46001",2),(1,"CHeN-UNICOM","UNICOM","46001",12),(3,"C
HINA MOBILE","CMCC","46000",7),(3,"CHN-CT","CT","46011",12),(3,"CHN-CT","CT","46011",7),
t
(3,"CHINA MOBILE","CMCC","46000",12),,(0-4),(0-2)
c
l
a
e
OK
AT+COPS? //Query the currently selected networki operator.
u t
+COPS: 0,0,"CHINA MOBILE",13
n
Q
OK e
d
3.1.2. AT+CREG Network Registration Status
i
f
The Read Command returns the network registration status and returns the status of result code
n
presentation and an integer <stat> which shows whether the network has currently indicated the
registration of MT. Loocation information parameters <lac> and <ci> are returned only when <n>=2 and
MT is registered on the network.
C
The Write Command sets whether to present URC or not and controls the presentation of an unsolicited
result code +CREG: <stat> when <n>=1 and there is a change in the MT network registration status.
AT+CREG Network Registration Status
Test Command Response
AT+CREG=? +CREG: (range of supported <n>s)
OK
Read Command Response
AT+CREG? +CREG: <n>,<stat>[,<lac>,<ci>[,<AcT>]]
OK
If there is any error related to MT functionality:
RG50xQ&RM5xxQ_Series_Network_Application_Note 13 / 136

                                                                5G Module Series

+CME ERROR: <err>
| Write Command          |     | Response  |     |     |
| ---------------------- | --- | --------- | --- | --- |
| AT+CREG=[<n>]          |     | OK        |     |     |
| Maximum Response Time  |     | 300 ms    |     |     |
| Characteristics        |     | /         |     |     |
| Reference              |     |           |     |     |
3GPP TS 27.007
Parameter

l
| <n>         | Integer type   |     |     |     |
| ----------- | -------------- | --- | --- | --- |
e
        0    Disable network registration unsolicited result code
Enable network registratiotn unsolicited result code: +CREG: <stat>
|     |    1    |     |     |     |
| --- | ------- | --- | --- | --- |
c
  2    Enable network registration unsolicited result code with location inforlmation:
a
 +CREG: <stat>[,<lac>,<ci>[,<AcT>]]
e
<stat>      Integer type. Indicate the circuit mode registration status.
i
        0    uNot registered. MT is not currently searching a newt operator to register to
|       |   1    | Registered, home network  |     | n   |
| ----- | ------ | ------------------------- | --- | --- |
Q
        2    Not registered, but MT is currently searching a new operator to register to
e
|       |   3     | Registration denied  |     |     |
| ----- | ------- | -------------------- | --- | --- |
|       |   4     | Unknown              | d   |     |
|       |    5    | Registered, roaming  |     |     |
Two bytes location area coide in hexadecimal format.
<lac>
f
| <ci>        |   28-bit (UMTS/LTE) cell ID in hexadecimal format.  |     |     |     |
| ----------- | --------------------------------------------------- | --- | --- | --- |
n
| <AcT>         | Integer type. Access technology selected.  |     |     |     |
| ------------- | ------------------------------------------ | --- | --- | --- |
  2 o      UTRAN
  4       UTRAN W/HSDPA (see NOTE 1)
C
  5       UTRAN W/HSUPA (see NOTE 1)
   6       UTRAN W/HSDPA and HSUPA (see NOTE 1)
|         |   7       E-UTRAN   |     |     |     |
| ------- | ------------------- | --- | --- | --- |
            10      E-UTRAN connected to a 5GCN (see NOTE 2) (not supported currently)
              11      NR connected to 5GCN (see NOTE 2) (not supported currently)
|                | 12      NG-RAN (not supported currently)             |     |     |     |
| -------------- | ---------------------------------------------------- | --- | --- | --- |
|                |   13      E-UTRAN-NR dual connectivity (see NOTE 3)  |     |     |     |
| <err>          | Error codes. See Chapter 1 for details.              |     |     |     |

NOTE
1.  3GPP TS 25.331 [74] specifies the System Information Blocks which give the information about
whether the serving cell supports HSDPA or HSUPA.
RG50xQ&RM5xxQ_Series_Network_Application_Note                                14 / 136

5G Module Series
2. 3GPP TS 38.331 [160] specifies the information which, if present, indicates that the serving cell is
connected to a 5GCN.
3. 3GPP TS 38.331 [160] specifies the information which, if present, indicates that the serving cell is
supporting dual connectivity of E-UTRA with NR and is connected to an EPS core.
Example
AT+CREG=1
OK
+CREG: 1 //URC reports that module has registered on network.
AT+CREG=2 //Activate extended URC mode.
l
OK e
t
+CREG: 1,"D509","80D413D",7 //URC reports that operator has found location area code and cell ID.
c
l
a
e
3.1.3. AT+CGREG Network Registration Status
i
u t
This command queries the network registration status and controls then presentation of an unsolicited result
Q
code +CGREG: <stat> when <n>=1 and there is a change in the MT’s GPRS network registration status
e
in GERAN/UTRAN, or unsolicited result code +CGREG: <stat>[,[<lac>],[<ci>],[<AcT>],[<rac>]] when
<n>=2 and there is a change of the network cell in GERAN/UTRAN.
d
AT+CGREG Network Registration Status
i
f
Test Command Response
n
AT+CGREG=? +CGREG: (range of supported <n>s)
o
OK
C
Read Command Response
AT+CGREG? +CGREG: <n>,<stat>[,[<lac>],[<ci>],[<AcT>],[<rac>]]
OK
Write Command Response
AT+CGREG=[<n>] OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics /
Reference
3GPP TS 27.007
RG50xQ&RM5xxQ_Series_Network_Application_Note 15 / 136

5G Module Series
Parameter
<n> Integer type.
0 Disable network registration unsolicited result code
1 Enable network registration unsolicited result code +CGREG: <stat>
2 Enable network registration and location information unsolicited result code
+CGREG: <stat>[,[<lac>],[<ci>],[<AcT>],[<rac>]]
<stat> Integer type. Indicate the GPRS registration status.
0 Not registered, MT is not currently searching an operator to register to. The UE is
in GMM state GMM-NULL or GMM-DEREGISTERED-INITIATED. The GPRS
service is disabled; the UE is allowed to attach for GPRS if requested by the user.
1 Registered, home network. The UE is in GMM state G MM-REGISTERED or
GMM-ROUTING-AREA-UPDATING-INITIATED INlITIATED on the home PLMN.
e
2 Not registered, but MT is currently trying to attach or searching an operator to
register to. The UE is in GMM state GMM-DEREGISTERED or
t
GMM-REGISTERED-INITIcATED. The GPRS service is enabled, but an allowable
l
PLMN is currently not available. The UE will start a GPRS attach asa soon as an
e
allowable PLMN is available.
i
3 Registruation denied. The UE is in GMM state GMM-NULL. T
t
he GPRS service is
disabled; and the UE is not allowed to attach for GnPRS if requested by the user.
Q4 Unknown
e
5 Registered, roaming
<lac> String type. Two-byte location area code in hexadecimal format (e.g., "00C3" equals 195 in
d
decimal).
<ci> String type. Four-byte (UMTS/LiTE) cell ID in hexadecimal format.
f
<AcT> Access technology selected.
n
2 UTRAN
4 UTRAN W/HSDPA
o
5 UTRAN W/HSUPA
C 6 UTRAN W/HSDPA and HSUPA
<rac> One byte routing area code in hexadecimal format.
Example
AT+CGREG=?
+CGREG: (0-2)
OK
AT+CGREG=2
OK
AT+CGREG?
+CGREG: 2,1,"D5D5","8054BBF",2,"0"
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 16 / 136

5G Module Series
+CGREG: 1,"D5D5","8054BBF",2,"0"
3.1.4. AT+CEREG EPS Network Registration Status
This command queries the network registration status and controls the presentation of an unsolicited result
code +CEREG: <stat> when <n>=1 and there is a change in the MT’s EPS network registration status in
E-UTRAN, or unsolicited result code +CEREG: <stat>[,[<tac>],[<ci>],[<AcT>]] when <n>=2 and there is
a change of the network cell in E-UTRAN.
AT+CEREG EPS Network Registration Status
Test Command Response
l
AT+CEREG=? +CEREG: (rangee of supported <n>s)
t
OK
c
l
Read Command Response
a
AT+CEREG? e+CEREG: <n>,<stat>[,<tac>,<ci>[,<AcT>]]
i
u t
OK
n
Write CommaQnd Response
AT+CEREG=[<n>] OK e
Or
d
ERROR
Maximum Response Time 300i ms
f
Characteristics n/
Reference
o
3GPP TS 27.007
C
Parameter
<n> Integer type.
0 Disable network registration unsolicited result code
1 Enable network registration unsolicited result code +CEREG: <stat>
2 Enable network registration and location information unsolicited result code
+CEREG: <stat>[,[<tac>],[<ci>],[<AcT>]]
<stat> Integer type. Indicate the EPS registration status.
0 Not registered, MT is not currently searching an operator to register to
1 Registered, home network
2 Not registered, but MT is currently trying to attach or searching an operator to
register to
3 Registration denied
4 Unknown
RG50xQ&RM5xxQ_Series_Network_Application_Note 17 / 136

5G Module Series
5 Registered, roaming
<tac> String type. Two-byte tracking area code in hexadecimal format.
<ci> String type. Four-byte (E-UTRAN) cell ID in hexadecimal format.
<AcT> Access technology selected.
7 E-UTRAN
13 E-UTRAN-NR dual connectivity
Example
AT+CEREG=?
+CEREG: (0-2)
l
OK
e
AT+CEREG=2
OK t
AT+CEREG? c
l
+CEREG: 2,1,"DE10","5A29C0B",7 a
e
i
OK u t
n
Q
+CEREG: 1,"DE10","5A29C0B",7
e
d
3.1.5. AT+C5GREG 5GS Network Registration Status
i
This command queries the network regisftration status and controls the presentation of URC +C5GREG:
<stat> when <n>=1 and there nis a change in the module's network registration status in 5GS, or URC
+C5GREG: <stat>[,[<tac>],[<ci>],[<AcT>],[<Allowed_NSSAI_length>],[<Allowed_NSSAI>]] when
o
<n>=2 and there is a change of the network cell in 5GS or the network provided an Allowed NSSAI. The
parameters <AcT>, <tac>, <ci>, <Allowed_NSSAI_length> and <Allowed_NSSAI> are provided only if
C
available.
AT+C5GREG 5GS Network Registration Status
Test Command Response
AT+C5GREG=? +C5GREG: (range of supported <n>s)
OK
Read Command Response
AT+C5GREG? +C5GREG: <n>,<stat>[,[<tac>],[<ci>],[<AcT>],[<Allowed
_NSSAI_length>],[<Allowed_NSSAI>]]
OK
Write Command Response
AT+C5GREG=[<n>] OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 18 / 136

                                                                5G Module Series

Or
ERROR
Maximum Response Time  300 ms
Characteristics  /
Reference
3GPP TS 27.007
Parameter
| <n>     | Integer type.  |     |     |
| ------- | -------------- | --- | --- |

      0      Disable network registration unsolicited resultl code
Enable network registration unsolicieted result code +C5GREG: <stat>
|       | 1      |     |     |
| ----- | ------ | --- | --- |
      2    Enable network registration and location information unsolicited result code
t
          +C5GREG: <stat>[,[<tac>],[<ci>],[<AcT>],[<Allowed_NSSAI_length>],[<Allo
c
l
|       |     wed_NSSAI>]]  |     |     |
| ----- | ----------------- | --- | --- |
a
| <stat>     | Integer type. Indicate the NR registration status.  | e   |     |
| ---------- | --------------------------------------------------- | --- | --- |
0       Not registered, the module is not currently searching an opeirator to register to
|       |     |     |     |
| ----- | --- | --- | --- |
u t
|     |    1       Registered, home network  |     |     |
| --- | ------------------------------------ | --- | --- |
n
           Q 2       Not registered, but the module is currently trying to attach or searching an
|       |     operator to register to  |     | e   |
| ----- | ---------------------------- | --- | --- |
|       | 3       Registration denied  |     |     |
d
|       | 4       Unknown              |     |     |
| ----- | ---------------------------- | --- | --- |
|       | 5       Registered, roaming  |     |     |
i
|             8      | Registered for emfergency services only  |     |     |
| ------------------ | ---------------------------------------- | --- | --- |
<tac>       String type. Threne-byte tracking area code in hexadecimal format.
<ci>        String type. Five-byte (NR) cell ID in hexadecimal format.
o
<AcT>      Access technology selected.
|       C           | 10    E-UTRAN connected to a 5GCN  |     |     |
| ----------------- | ---------------------------------- | --- | --- |
|             11    | NR connected to a 5GCN             |     |     |
<Allowed_NSSAI_length>  Integer type. Indicates the number of octets of the <Allowed_NSSAI>
|       |       |   information element.  |     |
| ----- | ----- | ----------------------- | --- |
<Allowed_NSSAI>      String type in hexadecimal format. Dependent of the form, the string can
            be separated by dot(s), semicolon(s) and colon(s). This parameter
            indicates the list of allowed S-NSSAIs received from the network. The
            <Allowed_NSSAI> is coded as a list of <S-NSSAI>s separated by
            colons. See <S-NSSAI> in subclause 10.1.1. This parameter shall not be
            subject to conventional character conversion as per AT+CSCS.
Example
AT+C5GREG=?
+C5GREG: (0-2)
RG50xQ&RM5xxQ_Series_Network_Application_Note                                19 / 136

5G Module Series
OK
AT+C5GREG=2
OK
AT+C5GREG?
+C5GREG: 2,1,"690E0F","9013B004",11,4,"01.000000"
OK
+C5GREG: 1,"690E0F","9013B004",11,4,"01.000000"
3.2. Network Paramenter Status
l
e
3.2.1. AT+QNWINFO Query Network Information
t
c
l
This command queries network information such as access technology selected, the operator and the
a
e
band selected.
i
AT+QNWINFO Query u Network Information t
n
Test CommaQnd Response
AT+QNWINFO=? OK e
Execution Command Response
d
AT+QNWINFO +QNWINFO: <AcT>,<oper>,<band>,<channel>
i[+QNWINFO: <AcT>,<oper>,<band>,<channel>]
f
n
OK
o
Maximum Response Time 300 ms
CharacteCristics /
Parameter
<AcT> String type. Access technology selected.
"NONE"
"WCDMA"
"TDD LTE"
"FDD LTE"
"TDD NR5G"
"FDD NR5G"
<oper> Operator names in numeric format.
<band> String type. Selected band.
"WCDMA_I_2100"
RG50xQ&RM5xxQ_Series_Network_Application_Note 20 / 136

5G Module Series
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
"LTE BAND 66"–"LTE BAND 68" l
e
"LTE BAND 71"
"LTE BAND 125"–"LTE BAND 127"
t
"LTE BAND 250" c
l
"LTE BAND 252" a
e
"LTE BAND 255"
i
"NR5Gu BAND 1"–"NR5G BAND 3"
t
"NR5G BAND 5" n
Q "NR5G BAND 7"–"NR5G BAND 8"
e
"NR5G BAND 12"
"NR5G BAND 14"
d
"NR5G BAND 20"
"NR5G BAND 25" i
f
"NR5G BAND 28"
n
"NR5G BAND 34"
"NR5G BAND 38"–"NR5G BAND 41"
o
"NR5G BAND 48"
C "NR5G BAND 50"–"NR5G BAND 51"
"NR5G BAND 65"–"NR5G BAND 66"
"NR5G BAND 70"–"NR5G BAND 71"
"NR5G BAND 74"–"NR5G BAND 86"
"NR5G BAND 257"–"NR5G BAND 261"
<channel> Integer type. Channel ID.
NOTE
If the devices have not been registered on a network, the command returns +QNWINFO: No Service.
For NR5G NSA, it returns both LTE and NR5G information.
Example
AT+QNWINFO=?
RG50xQ&RM5xxQ_Series_Network_Application_Note 21 / 136

5G Module Series
OK
AT+QNWINFO
+QNWINFO: "FDD LTE","46001","LTE BAND 3",1650
OK
3.2.2. AT+QENG Query Primary Serving Cell and Neighbor Cell Information
This command obtains the network information, such as serving cell and neighbor cells.
AT+QENG Query Primary Serving Cell and Neighbor Cell Information
Test Command Response
l
AT+QENG=? +QENG: (list of seupported <cell_type>s)
t
OK
c
l
Write Command Response
a
Query the serving cell information eIn SA mode:
AT+QENG="servingcell" +QENG: "servingcell",<state>,"NR5Gi-SA",<duplex_mod
u t
e>,<MCC>,<MNC>,<cellID>,<PCID>,<TAC>,<ARFCN>,<ba
n
nd>,<NR_DL_bandwidth>,<RSRP>,<RSRQ>,<SINR>,<sc
Q
s>,<srxlev>
e
OK d
i
fIn EN-DC mode:
n+QENG: "servingcell",<state>
+QENG: "LTE",<is_tdd>,<MCC>,<MNC>,<cellID>,<PCI
o
D>,<earfcn>,<freq_band_ind>,<UL_bandwidth>,<DL_ban
dwidth>,<TAC>,<RSRP>,<RSRQ>,<RSSI>,<SINR>,<CQI>,
C
<tx_power>,<srxlev>
+QENG: "NR5G-NSA",<MCC>,<MNC>,<PCID>,<RSRP>,<
SINR>,<RSRQ>,<ARFCN>,<band>,<NR_DL_bandwidth>,
<scs>
OK
In LTE mode:
+QENG: "servingcell",<state>,"LTE",<is_tdd>,<MCC>,<
MNC>,<cellID>,<PCID>,<earfcn>,<freq_band_ind>,<UL_b
andwidth>,<DL_bandwidth>,<TAC>,<RSRP>,<RSRQ>,<R
SSI>,<SINR>,<CQI>,<tx_power>,<srxlev>
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 22 / 136

5G Module Series
In WCDMA mode:
+QENG: "servingcell",<state>,"WCDMA",<MCC>,<MN
C>,<LAC>,<cellID>,<uarfcn>,<PSC>,<RAC>,<RSCP>,<eci
o>,<phych>,<SF>,<slot>,<speech_code>,<comMod>
OK
Write Command Response
Query the information of neighbor cells In LTE mode:
AT+QENG="neighbourcell" [+QENG: "neighbourcell intra","LTE",<earfcn>,<PCID>,
<RSRQ>,<RSRP>,<RSSI>,<SINR>,<srxlev>,<cell_resel_p
riority>,<s_non_intra_search>,<thresh_serving_low>,<s
_intra_search>
l
…]
e
[+QENG: "neighbourcell inter","LTE",<earfcn>,<PCID>,
<RSRQ>,<RtSRP>,<RSSI>,<SINR>,<srxlev>,<cell_resel_p
riocrity>,<threshX_low>,<threshX_high>
l
…] a
e
[+QENG:"neighbourcell","WCDMA",<uarfcn>,<cell_resel
i
u_priority>,<thresh_Xhigh>,<thres
t
h_Xlow>,<PSC>,<RSC
P><ecno>,<srxlev> n
Q
…]
e
In WCDMA mode:
d
[+QENG:"neighbourcell","WCDMA",<uarfcn>,<srxqual>,
<PiSC>,<RSCP>,<ecno>,<set>,<rank>,<srxlev>
f
…]
n
[+QENG: "neighbourcell","LTE",<earfcn>,<PCID>,<RSR
o P>,<RSRQ>,<s_rxlev>
…]
C
OK
Maximum Response Time 300 ms
Characteristics /
Parameter
<cell_type> String type. The information of different cells.
"servingcell" The information of 3G/4G/5G serving cells
"neighbourcell" The information of 3G/4G neighbor cells
<state> String type. UE state.
"SEARCH" UE is searching but could not (yet) find a suitable 3G/4G/5G cell.
"LIMSRV" UE is camping on a cell but has not registered on the network.
RG50xQ&RM5xxQ_Series_Network_Application_Note 23 / 136

5G Module Series
"NOCONN" UE is camping on a cell and has registered on the network, and it
is in idle mode.
"CONNECT" UE is camping on a cell and has registered on the network, and a
call is in progress.
<duplex_mode> String type. The NR5G network mode.
"TDD"
"FDD"
<is_tdd> String type. The LTE network mode.
"TDD"
"FDD"
<MCC> 16-bit unsigned integer. Mobile Country Code (first part of the PLMN code).
<MNC> 16-bit unsigned integer. Mobile Network Code (seco nd part of the PLMN code).
<ARFCN> SA-ARFCN of the cell that was scanned. l
e
<band> 32-bit unsigned integer. Frequency band in NR5G SA network mode.
<NR_DL_bandwidth> Integer type. DL bandwidth.
t
0 5 MHz c
l
1 10 MHz a
e
2 15 MHz
i
u3 20 MHz
t
4 25 MHz n
Q5 30 MHz
e
6 40 MHz
7 50 MHz
d
8 60 MHz
9 70 MHZ i
f
10 80 MHz
n
11 90 MHz
12 100 MHz
o
13 200 MHz
C14 400 MHz
15 35 MHz
16 45 MHz
<LAC> Integer type. Location Area Code. The parameter determines the two bytes
location area code in hexadecimal format (e.g. 00C1 equals 193 in decimal)
of the cell that was scanned. Range: 0–65535.
<cellID> Integer type. Cell ID. The parameter determines the 28-bit (UMTS, LTE) or
36-bit (NR5G) cell ID. Range: 0–0xFFFFFFFFF.
<PCID> Number format. Physical cell ID.
<uarfcn> UTRA-ARFCN of the cell that was scanned.
<earfcn> E-UTRA-ARFCN of the cell that was scanned.
<freq_band_ind> Integer type. E-UTRA frequency band (see 3GPP 36.101)
<UL_bandwidth> Integer type. UL bandwidth.
0 1.4 MHz
1 3 MHz
RG50xQ&RM5xxQ_Series_Network_Application_Note 24 / 136

5G Module Series
2 5 MHz
3 10 MHz
4 15 MHz
5 20 MHz
<DL_bandwidth> Integer type. DL bandwidth.
0 1.4 MHz
1 3 MHz
2 5 MHz
3 10 MHz
4 15 MHz
5 20 MHz
<TAC> Tracking Area Code (see 3GPP 23.003 Sec tion 19.4.2.3)
<PSC> Primary scrambling code of the cell that lwas scanned
e
<RAC> Integer type. Routing Area Code. Range: 0–255.
<RSCP> Received Signal Code Power level of the cell that was scanned.
t
<ecio> Carrier to noise rcatio in dB = measured Ec/Io value in dB.
l
<RSRP> In LTE mode: a
e
Signal of LTE Reference Signal Received Power (see 3GPP 36.214).
i
uRange: -140 to -44 dBm. The closer to -44, the
t
better the signal is. The
closer to -140, the worse the signal is. n
QIn NR5G mode:
e
Signal of NR5G Reference Signal Received Power. Range: -140 to -44 dBm.
The closer to -44, the better the signal is. The closer to -140, the worse the
d
signal is.
<RSRQ> In LTE mode: i
f
Signal of current LTE Reference Signal Received Quality (see 3GPP
n
36.214). Range: -20 to -3 dB. The closer to -3, the better the signal is. The
closer to -20, the worse the signal is.
o
In NR5G mode:
CSignal of current NR5G Reference Signal Received Quality. Range: -20 to -
3 dB. The closer to -3, the better the signal is. The closer to -20, the worse
the signal is.
<RSSI> LTE Received Signal Strength Indication.
<SINR> In LTE mode:
LTE Signal-to-Interface plus Noise Ratio. The conversion formula for actual
SINR is Y = (1/5) × X × 10 - 20 (X is <SINR> queried by AT+QENG. Y is the
actual value of LTE SINR after calculating with the formula). Range: -20 to
30 dB.
In NR5G mode:
Signal of NR5G Signal-to-Interface plus Noise Ratio. Range: -23 to 40 dB.
<CQI> Integer type. Channel Quality Indication. Range: 1–30.
<tx_power> TX power value in 1/10 dBm. It is the maximum of all UL channel TX power.
<tx_power> is only meaningful when the device is in traffic.
<phych> Integer type. Physical channel.
RG50xQ&RM5xxQ_Series_Network_Application_Note 25 / 136

5G Module Series
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
<slot> Integer type. l
e
0–16 Slot format for DPCH.
0–9 Slot format for FDPCH
t
<speech_code> Destination numbcer on which call is to be deflected.
l
<comMod> Integer type. Number format. Compress mode. a
e
0 Not support compress mode
i
u1 Support compress mode
t
<srxqual> Receiver automatic gain control on the cnamped frequency.
<ecno> QInteger type. Carrier to noise ratio in dB = measured Ec/Io value in dB.
e
<set> Integer type. 3G neighbor cell set.
1 Active set
d
2 Synchronous neighbor set
3 Asynchronouis neighbor set
f
<rank> Rank of this cell as neighbor for inter-RAT cell reselection.
n
<srxlev> Suitable reception level for inter frequency cell.
<threshX_low> To be considered for re-selection. The suitable receive level value of an
o
evaluated lower priority cell must be greater than this value.
<threshXC_high> To be considered for re-selection. The suitable receive level value of an
evaluated higher priority cell must be greater than this value.
<thresh_Xhigh> Reselection threshold for high priority layers.
<thresh_Xlow> Reselection threshold for low priority layers.
<srxlev> Select reception level value for base station in dB (see 3GPP 25.304).
<cell_resel_priority> Integer type. Cell reselection priority. Range: 0–7.
<s_non_intra_search> Threshold to control non-intra frequency searches.
<thresh_serving_low> Specifies the suitable reception level threshold (in dB) used by the UE on
the serving cell when reselecting towards a lower priority RAT/frequency.
<s_intra_search> Cell selection parameter for the intra frequency cell.
<scs> Integer type. NR sub carrier spacing.
0 15 kHz
1 30 kHz
2 60 kHz
3 120 kHz
RG50xQ&RM5xxQ_Series_Network_Application_Note 26 / 136

5G Module Series
4 240 kHz
NOTE
"-" or - indicates the parameter is invalid under current condition.
Example
AT+QENG="servingcell"
+QENG: "servingcell","NOCONN","LTE","FDD",460,01,5F1EA15,12,1650,3,5,5,DE10,-100,-12,-68,1
1,0,-32768,27
l
AT+QENG="servingcell"
e
+QENG: "servingcell","NOCONN"
+QENG: "LTE","FDD",460,01,5F1EA15,12,1650,3,5,t5,DE10,-99,-12,-67,11,9,230,-
c
+QENG:"NR5G-NSA",460,01,747,-71,33,-11,627264,78,12,1 l
a
AT+QENG="servingcell"
e
+QENG: "servingcell","NOCONN","NR5G-SA","TDD",460,01,19013B004,299,690E0F,633984,78,12,
i
-107,-13,2,1,- u t
n
Q
OK
e
AT+QENG="neighbourcell"
+QENG: "neighbourcell intra","LTE",38950,276d,-3,-88,-65,0,37,7,16,6,44
+QENG: "neighbourcell inter","LTE",39148,-,-,-,-,-,37,0,30,7,-,-,-,-
i
+QENG: "neighbourcell inter","LTE",37900,-,-,-,-,-,0,0,30,6,-,-,-,-
f
n
OK
o
3.2.3. ACT+QCAINFO Query Carrier Aggregation Parameters
This command queries carrier aggregation parameters.
AT+QCAINFO Query Carrier Aggregation Parameters
Test Command Response
AT+QCAINFO=? +QCAINFO: (list of supported <5G_signal_ext>)
OK
Read Command Response
AT+QCAINFO? +QCAINFO: <5G_signal_ext>
OK
Write Command Response
AT+QCAINFO=<5G_signal_extend> OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 27 / 136

5G Module Series
Or
ERROR
Execution Command In LTE mode:
AT+QCAINFO +QCAINFO: "PCC",<freq>,<bandwidth>,<band>,<pcell_s
tate>,<PCID>,<RSRP>,<RSRQ>,<RSSI>,<RSSNR>
[+QCAINFO: "SCC",<freq>,<bandwidth>,<band>,<scell_
state>,<PCID>,<RSRP>,<RSRQ>,<RSSI>,<RSSNR><UL_
configured>,<UL_bandwidth>,<UL_EARFCN>]
[…]
OK
l
In EN-DC mode:
e
+QCAINFO: "PCC",<freq>,<bandwidth>,<band>,<pcell_s
tate>,<PCIDt>,<RSRP>,<RSRQ>,<RSSI>,<RSSNR>
[+QcCAINFO: "SCC",<freq>,<bandwidth>,<band>,<scell_
l
state>,<PCID>,<RSRP>,<RSRQ>,<RSSI>,a<RSSNR><UL_
e
configured>,<UL_bandwidth>,<UL_EARFCN>]
i
u[…]
t
[+QCAINFO: "SCC",<frenq>,<NR_DL_bandwidth>,<NR_b
Q
and>,<PCID>]
e
[+QCAINFO: "SCC",<freq>,<NR_DL_bandwidth>,<NR_b
and>,<state>,<PCID>,<UL_configured>,<NR_UL_bandwi
d
dth>,<UL_ARFCN>[,<NR_RSRP>,<NR_RSRQ>]
[…i]
f
n
OK
o
In SA mode:
C
+QCAINFO: "PCC",<freq>,<NR_DL_bandwidth>,<NR_ba
nd>,<PCID>
[+QCAINFO: "SCC",<freq>,<NR_DL_bandwidth>,<NR_b
and>,<state>,<PCID>,<UL_configured>,<NR_UL_bandwi
dth>,<UL_ARFCN>[,<NR_RSRP>,<NR_RSRQ>]
[…]
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
RG50xQ&RM5xxQ_Series_Network_Application_Note 28 / 136

                                                                5G Module Series

Parameter
| <PCC>              |     | Primary carrier component.    |          |
| ------------------ | --- | ----------------------------- | -------- |
| <SCC>              |     | Secondary carrier component.  |          |
| <freq>             |     | EARFCN.                       |          |
| <bandwidth>        |     | Integer type. Bandwidth.      |          |
|                    |     | 6                             | 1.4 MHz  |
|                    |     |   15                          | 3 MHz    |
|                    |     |   25                          | 5 MHz    |
|                    |     |   50                          | 10 MHz   |
|                    |     |   75                          | 15 MHz   |
|                    |     |   100                         | 20 MHz   |

l
| <band>           |     | String type. DL Band information.  |     |
| ---------------- | --- | ---------------------------------- | --- |
e
                    "LTE BAND 1"
                    "LTE BAND 2"  t
                    "LTE BAND 3"  c
l
                    …  a
e
                    "LTE BAND 66"
i
| <pcell_state>      |     | Integer type. Primary cell state.  |     |
| ------------------ | --- | ---------------------------------- | --- |
u t
|     |       | 0  Not registered, not searching  |     |
| --- | ----- | --------------------------------- | --- |
n
|     |   Q    | 1  Registered on home network  |     |
| --- | ------ | ------------------------------ | --- |
e
|     |       | 2  Not registered, searching  |     |
| --- | ----- | ----------------------------- | --- |
|     |       | 3  Registration denied        |     |
d
|     |       | 4  Unknow registration state       |     |
| --- | ----- | ---------------------------------- | --- |
|     |       | 5  Registered on roaiming network  |     |
f
<scell_state>       Integer type. Secondary cell state.
n
|     |     | 0    | Deconfigured               |
| --- | --- | ---- | -------------------------- |
|     |     | 1    | Configuration deactivated  |
o
    2       Configuration activated
<PCID>C             Integer type. Physical Cell ID.
<RSRP>            Integer type. Reference Signal Received Power (see 3GPP 36.214)
<RSRQ>            Integer type. Reference Signal Received Quality (see 3GPP 36.214)
<RSSI>             Integer type. Received Signal Strength Indication.
<RSSNR>           Integer type. Logarithmic value of RSSNR. Range: -10 to +30 dB.
<UL_configured>  Integer type. Whether the UL of secondary cell is configured by network.
|     |     | 0  Not configured  |     |
| --- | --- | ------------------ | --- |
|     |     | 1  Configured      |     |
<UL_bandwidth>  Integer type. UL bandwidth. "-" will be displayed if <UL_configured>=0.
|     |     | 6       | 1.4 MHz  |
| --- | --- | ------- | -------- |
|     |     |   15    | 3 MHz    |
|     |     |   25    | 5 MHz    |
|     |     |   50    | 10 MHz   |
|     |     |   75    | 15 MHz   |
|     |     | 100     | 20 MHz   |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                29 / 136

                                                                5G Module Series

<UL_EARFCN>    Integer type. UL EARFCN. "-" will be displayed if <UL_configured>=0.
<NR_DL_bandwidth>Integer type. NR downlink bandwidth.
|     | 0    | 5 MHz   |     |
| --- | ---- | ------- | --- |
|     | 1    | 10 MHz  |     |
|     | 2    | 15 MHz  |     |
|     | 3    | 20 MHz  |     |
|     | 4    | 25 MHz  |     |
|     | 5    | 30 MHz  |     |
|     | 6    | 40 MHz  |     |
|     | 7    | 50 MHz  |     |
|     | 8    | 60 MHz  |     |
|     | 9    | 70 MHz  |     |

l
|     | 10    | 80 MHz  |     |
| --- | ----- | ------- | --- |
e
|     | 11    | 90 MHz   |     |
| --- | ----- | -------- | --- |
|     | 12    | 100 MHz  | t   |
|     | 13    | 200 MHz  | c   |
l
|     | 14    | 400 MHz  | a   |
| --- | ----- | -------- | --- |
e
|     | 15      35 MHz  |     |     |
| --- | --------------- | --- | --- |
i
|     | 16u      45 MHz  |     |     |
| --- | ---------------- | --- | --- |
t
| <NR_band>  |     String Type. DL Band information.  |     |     |
| ---------- | -------------------------------------- | --- | --- |
n
Q
|     |   "NR5G BAND 1"  |     |     |
| --- | ---------------- | --- | --- |
e
|     |     "NR5G BAND 2"  |     |     |
| --- | ------------------ | --- | --- |
|     |     "NR5G BAND 3”  |     |     |
d
|     |     …                |     |     |
| --- | -------------------- | --- | --- |
|     |     "NR5G BAND 261"  |     | i   |
f
<NR_UL_bandwidth>Integer type. "-" will be displayed if <UL_configured>=0. The value of
n
          <NR_UL_bandwidth> is the same as that of <NR_DL_bandwidth>.
<UL_ARFCN>     Integer type. UL_ARFCN. "-" will be displayed if <UL_configured> is 0.
o
<NR_RSRP>       Signal of NR5G reference signal received power. Range: -140 to -44. Unit: dBm.
C
  The closer to -44, the better the signal is. The closer to -140, the worse the signal
    is.
<NR_RSRQ>       Signal of current NR5G reference signal received quality. Range: -20 to -3. Unit:
          dB. The closer to -3, the better the signal is. The closer to -20, the worse the signal
|       |     is.  |     |     |
| ----- | -------- | --- | --- |
<5G_signal_ext>    Integer type. Hide or show extension parameters <NR_RSRP>, and <NR_RSRQ>
|     |   0    |   Hide  |     |
| --- | ------ | ------- | --- |
|     |   1    |   Show  |     |

NOTE
This command is valid only after the network is registered.
RG50xQ&RM5xxQ_Series_Network_Application_Note                                30 / 136

                                                                5G Module Series

Example
AT+QCAINFO
+QCAINFO: "PCC",300,100,"LTE BAND 1",1,23,-66,-12,-34,30
+QCAINFO: "SCC",1575,100,"LTE BAND 3",2,43,-64,-7,-24,30,0,-,-

OK

3.2.4. AT+QENDC  Query EN-DC Status
This command queries EN-DC status.

| AT+QENDC  Query EN-DC Status  |     |     |     | l   |
| ----------------------------- | --- | --- | --- | --- |
e
| Read Command  |     |     | Response         |     |
| ------------- | --- | --- | ---------------- | --- |
| AT+QENDC?     |     |     | +QENDC: <mtode>  |     |
  c
l
OK  a
e
| Write Command  |     |     | Response  |     |
| -------------- | --- | --- | --------- | --- |
i
| AT+QENDC=<mode>    |     | uOK  |           | t   |
| ------------------ | --- | ---- | --------- | --- |
| Execution Command  |     |      | Response  | n   |
Q
AT+QENDC  +QENDC: <endc_avl>,<plmn_info_list_r15_avl>,<endc_r
e
str>,<5G_basic>,<5G_UWB>

d
OK
i
| Maximum Response Time  |     |     | 300 ms  |     |
| ---------------------- | --- | --- | ------- | --- |
f
nThe command takes effect immediately.
Characteristics
The configuration is saved automatically.
o
C
Parameter
<endc_avl>              Integer type. Indicate whether the current cell supports EN-DC mode.
|       |     |     0  Not support  |     |     |
| ----- | --- | ------------------- | --- | --- |
|       |     |     1  Support      |     |     |
<plmn_info_list_r15_avl>    Integer type. Indicate whether the currently registered PLMN supports
|       |     |     the EN-DC mode.  |     |     |
| ----- | --- | -------------------- | --- | --- |
|       |     |     0  Not support   |     |     |
|       |     |     1  Support       |     |     |
<endc_rstr>             Integer type. EN-DC capability delivered by the network.
|       |     |     0  Not Restricted  |     |     |
| ----- | --- | ---------------------- | --- | --- |
|       |     |     1  Restricted      |     |     |
<5G_basic>               Integer type. Indicate whether to show 5G Icon information.
|       |     |     0  Not show  |     |     |
| ----- | --- | ---------------- | --- | --- |
|       |     | 1  Show          |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                31 / 136

5G Module Series
<5G_UWB> Integer type. Indicate whether to show 5G UWB Icon information.
0 Not show
1 Show
<mode> Integer type. Disable or disable the following URC:
+QENDC: <endc_avl>,<plmn_info_list_r15_avl>,<endc_rstr>,<5G_b
asic>,<5G_UWB>
0 Disable
1 Enable
Example
AT+QENDC
l
+QENDC: 1,1,0,1,0
e
OK t
AT+QENDC=1 //Enable ENDC URC c
l
OK a
e
i
// URC Report u t
+QENDC: 1,1,0,0,0 n
Q
e
+QENDC: 1,1,0,1,0
d
3.2.5. AT+QSCAN Search Nearby Cells
i
f
This command searches nearbny LTE cells and NR5G cells.
AT+QSCAN Seaorch Nearby Cells
Test Command Response
C
AT+QSCAN=? +QSCAN: (range of supported <mode>s)
OK
Write Command Response
AT+QSCAN=<mode>[,<ext>[,<scan_ [+QSCAN: "LTE",<MCC>,<MNC>,<freq>,<PCI>,<RSRP>,<
LTE_band>[,<scan_NR5G_band>[,< RSRQ>,<srxlev>,<squal>[,<cellID>,<TAC>,<bandwidth>,<
delay_time>]]]] LTE_band>[,<short_name>,<full_name>]]]
…]
[+QSCAN: "NR5G",<MCC>,<MNC>,<freq>,<PCI>,<RSR
P>,<RSRQ>,<srxlev>,<SCS>[,<cellID>,<TAC>,<carrierBan
dwidth>,<band>,<offsetToPointA>,<SSB_subcarrier_offs
et>,<SSB_SCS>,[<short_name>,<full_name>]]
…]
RG50xQ&RM5xxQ_Series_Network_Application_Note 32 / 136

                                                                5G Module Series

OK

If there is any error:
ERROR

If there is any error related to MT functionality:
+CME ERROR:<err>
Maximum Response Time  360 s, determined by the network.
Characteristics  /

Parameter
l
e
| <mode>   |       |   Integer type. Cell searching mode.  |     |     |
| -------- | ----- | ------------------------------------- | --- | --- |
|          |       |   1  Search only for LTtE cells       |     |     |
Search ocnly for NR5G cells
|       |       |   2  |     | l   |
| ----- | ----- | ---- | --- | --- |
Search for LTE cells and NR5G cells at the same taime
|       |       |   3  |     |     |
| ----- | ----- | ---- | --- | --- |
e
<ext>            Integer type. Hide or show the extension parameter options or  show the
i
            u  extension parameters (support 5G cell withoutt tac).
|       |       |   0  Hide extension parameters  | n   |     |
| ----- | ----- | ------------------------------- | --- | --- |
Q
              1  Show extension parameters (<cellID>, <TAC>, <bandwidth>,
e
                <LTE_band>, <carrierBandwidth>, <band>, <offsetToPointA>,
|       |       |      <SSB_subcarrier_offset> and <SSB_SCS>)  |     |     |
| ----- | ----- | -------------------------------------------- | --- | --- |
d
              2  Show extension parameters and support 5G cell without tac
(<cellID>i, <TAC>, <bandwidth>, <LTE_band>,
|       |       |      |     |       |
| ----- | ----- | ---- | --- | ----- |
f
                <carrierBandwidth>, <band>, <offsetToPointA>,
n
|       |       |     <SSB_subcarrier_offset> and <SSB_SCS>)  |     |     |
| ----- | ----- | ------------------------------------------- | --- | --- |
          o      At this time, <mode> must be 2, otherwise error is returned
              3  Show extension parameters (<cellID>, <TAC> ,<bandwidth>,
C
                <LTE_band>, <carrierBandwidth>, <band>, <offsetToPointA>,
                <SSB_subcarrier_offset>, <SSB_SCS>, <full_name> and
|       |       |     <short_name>)  |     |     |
| ----- | ----- | ------------------ | --- | --- |
<MCC>           Integer type. Mobile Country Code (first part of the PLMN code).
<MNC>           Integer type. Mobile Network Code (second part of the PLMN code).
| <freq>    |       |   Integer type. Cell frequency.    |     |     |
| --------- | ----- | ---------------------------------- | --- | --- |
| <PCI>     |       |   Integer type. Physical Cell ID.  |     |     |
<RSRP>        Integer type. It indicates the signal of Reference Signal Received Power
|     |     |   (see 3GPP 36.214). Range: -140 to -44 dBm.  |     |     |
| --- | --- | --------------------------------------------- | --- | --- |
<RSRQ>        Integer type. It indicates the signal of current Reference Signal Received
|     |     |   Quality (see 3GPP 36.214). Range: -20 to -3 dB.  |     |     |
| --- | --- | -------------------------------------------------- | --- | --- |
<srxlev>          Integer type. Cell selection RX level value. Unit: dB.
<squal>           Integer type. Cell selection quality value. Unit: dB.
<cellID>        String type without double quotes. Cell Identity in hex string.
<TAC>        String type without double quotes. Tracking Area Code in hex string.
RG50xQ&RM5xxQ_Series_Network_Application_Note                                33 / 136

                                                                5G Module Series

| <bandwidth>  |     |     Integer type. Bandwidth value.    |          |     |     |
| ------------ | --- | ------------------------------------- | -------- | --- | --- |
|              |     |     6                                 | 1.4 MHz  |     |     |
|              |     |     15                                | 3 MHz    |     |     |
|              |     |     25                                | 5 MHz    |     |     |
|              |     |     50                                | 10 MHz   |     |     |
|              |     |     75                                | 15 MHz   |     |     |
|              |     |     100                               | 20 MHz   |     |     |
| <SCS>        |     |     Integer type. Sub-carrier space.  |          |     |     |
|              |     |     0  15 KHz                         |          |     |     |
|              |     |     1  30 KHz                         |          |     |     |
|              |     |     2  60 KHz                         |          |     |     |
|              |     |     3  120 KHz                        |          |     |     |
Integer type. Carrier bandwidth, the nlumber of the RBs in sub-carrier.
| <carrierBandwidth>  |     |     |     |     |     |
| ------------------- | --- | --- | --- | --- | --- |
e
| <band>                    |     |     Integer type. Frequency band indicator of NR.  |     |     |     |
| ------------------------- | --- | -------------------------------------------------- | --- | --- | --- |
| <offsetToPointA>          |     |     Integer type. Offset to tPoint A.              |     |     |     |
| <SSB_subcarrier_offset>   |     | Integer type. ScSB sub-carrier offset.             |     |     |     |
l
| <SSB_SCS>  |     |     Integer type. SSB SCS value.  |     |     | a   |
| ---------- | --- | --------------------------------- | --- | --- | --- |
e
|       |     |     0  15 KHz  |     |     |     |
| ----- | --- | -------------- | --- | --- | --- |
i
|       |     |   u  1  30 KHz  |     |     |     |
| ----- | --- | --------------- | --- | --- | --- |
t
|       |     |     2  60 KHz  |     | n   |     |
| ----- | --- | -------------- | --- | --- | --- |
Q
|       |     |     3  120 KHz  |     |     |     |
| ----- | --- | --------------- | --- | --- | --- |
e
<LTE_bandx>        Integer type. Frequency band indicator of LTE.
<scan_LTE_band>       String type without double quotes. Use the colon as a separator to list
d
              the LTE bands to be configured. The parameter format is:
|       |     |     <LTE_band1i>:<LTE_band2>:…:<LTE_bandx>.  |     |     |     |
| ----- | --- | -------------------------------------------- | --- | --- | --- |
f
<scan_NR5G_band>     String type without double quotes. Use the colon as a separator to list
n
              the NR5G bands to be configured. The parameter format is
|                |     o  |   <NR5G_band1>:<NR5G_band2>:…:<NR5G_bandx>.  |     |     |     |
| -------------- | ------ | -------------------------------------------- | --- | --- | --- |
| <NR5G_bandx>   |        |     Integer type. NR5G band.                 |     |     |     |
<delay_tCime>
      Integer type. Delay until the NSA sweep is over. Optional only when <ext>
              is 2. Unit: second. Range: 0–180. Default value: 40.
<full_name>         String type. The full name of the network operator. Only output when
|       |     |     <ext> is 3.   |     |     |     |
| ----- | --- | ----------------- | --- | --- | --- |
<short_name>        String type. The short name of the network operator. Only output when
|       |     |     <ext> is 3.  |     |     |     |
| ----- | --- | ---------------- | --- | --- | --- |

NOTE
1.  This command returns “–” for the parameters if UE cannot get the corresponding information.
2.  This command does not apply to 5G cells in NSA mode.
3.  This command is recommended to be used when there is no (U)SIM card.
4.  When <ext> is 2, <mode> must be 2; otherwise, error is returned.
5.  <delay_time> is a special optional extended parameter when <ext> is 2.
RG50xQ&RM5xxQ_Series_Network_Application_Note                                34 / 136

5G Module Series
Example
AT+QSCAN=1,1
+QSCAN: "LTE",460,00,3590,207,-128,-13,-1,115,848459E,550B,50,8
+QSCAN: "LTE",460,11,1850,378,-135,-20,-7,109,DD8A33F,691D,100,3
OK
AT+QSCAN=2,1
+QSCAN: "NR5G",460,00,504990,901,-95,-11,26,1,170C23000,46550B,273,41,30,6,1
+QSCAN: "NR5G",460,11,633984,441,-112,-13,9,1,690133003,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,01,633984,441,-112,-13,9,1,690133003,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,11,633984,223,-112,-15,9,1,69034E007,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,01,633984,223,-112,-15,9,1,69034E007,690E0F,273,78,28,4,1
l
e
OK
t
AT+QSCAN=1,1,1
c
+QSCAN: "LTE",001,01,100,1,-97,-13,43,116,1A2D001,1,100,1 l
a
e
OK i
AT+QSCAN=2,1,1,78 u t
n
+QSCAN: "NR5G",460,11,633984,841,-74,-11,46,1,6909DB085,690E0F,273,78,28,4,-
Q
+QSCAN: "NR5G",460,01,633984,841,-74,-11,46,1,6909DB085,690E0F,273,78,28,4,-
e
OK d
AT+QSCAN=3,1,3,1
i
+QSCAN: "NR5G",460,11,427210,810,-123,-16,-3,0,69067E483,690E0F,106,1,20,6,-
f
+QSCAN: "NR5G",460,01,427n210,810,-123,-16,-3,0,69067E483,690E0F,106,1,20,6,-
+QSCAN: "LTE",460,11,1850,378,-83,-13,46,115,DD8A33F,691D,100,3
o
+QSCAN: "LTE",460,00,1300,123,-75,-7,51,122,D6B5C0,550B,100,3
+QSCAN: "LTE",460,01,1506,157,-122,-17,6,112,5AC820C,DE10,50,3
C
+QSCAN: "LTE",460,01,1650,465,-85,-6,37,119,5A29C0B,DE10,100,3
OK
AT+QSCAN=2,2
+QSCAN: "NR5G",460,11,633984,841,-70,-11,50,1,6909DB085,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,01,633984,841,-70,-11,50,1,6909DB085,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,11,633984,223,-93,-12,28,1,69034E007,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,01,633984,223,-93,-12,28,1,69034E007,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,00,152650,30,-122,-15,-1,0,175E7A001,46550B,160,28,15,10,0
+QSCAN: "NR5G",460,15,152650,30,-122,-15,-1,0,175E7A001,46550B,160,28,15,10,0
+QSCAN: "NR5G",460,00,504990,631,-82,-11,38,1,170C23000,46550B,273,41,30,6,-
+QSCAN: "NR5G",460,15,504990,631,-82,-11,38,1,170C23000,46550B,273,41,30,6,-
OK
AT+QSCAN=2,2,,,40
RG50xQ&RM5xxQ_Series_Network_Application_Note 35 / 136

5G Module Series
+QSCAN: "NR5G",460,00,504990,631,-83,-11,38,1,170C23000,46550B,273,41,30,6,1
+QSCAN: "NR5G",460,15,504990,631,-83,-11,38,1,170C23000,46550B,273,41,30,6,1
+QSCAN: "NR5G",460,11,633984,841,-67,-11,53,1,6909DB085,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,01,633984,841,-67,-11,53,1,6909DB085,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,11,633984,223,-92,-12,28,1,69034E007,690E0F,273,78,28,4,1
+QSCAN: "NR5G",460,01,633984,223,-92,-12,28,1,69034E007,690E0F,273,78,28,4,1
+QSCAN: "NR5G",-,-,633984,441,-99,-14,-,0,-,-,-,-,-,-,-
+QSCAN: "NR5G",-,-,633984,0,-101,-15,-,0,-,-,-,-,-,-,-
OK
AT+QSCAN=2,3
+QSCAN:"NR5G",460,00,504990,631,-83,-11,38,1,170C23000,46550B ,273,41,30,6,-,"CMCC","CHINA
MOBILE" l
e
+QSCAN: "NR5G",460,15,504990,631,-83,-11,38,1,170C23000,46550B,273,41,30,6,-,"-","-"
+QSCAN:"NR5G",460,00,152650,30,-121,-14,-,0,175E7A001,46550B,160,28,15,10,0,"CMCC","CHIN
t
A MOBILE" c
l
+QSCAN: "NR5G",460,15,152650,30,-121,-14,-,0,175E7A001,46550B,160,28,15,10,0a,"-","-"
e
+QSCAN:"NR5G",460,11,633984,841,-68,-11,52,1,6909DB085,690E0F,273,78,28,4,1,"CT","CHN-CT"
i
+QSCAN:"NR5G",460,01,633984,841,-68,-11,52,1,6909DB085,690E0F,273,78,28,4,1,"UNICOM","CH
u t
N-UNICOM" n
+QSCAN: "NQR5G",460,11,633984,223,-92,-12,28,1,69034E007,690E0F,273,78,28,4,1,"CT","CHN-CT
" e
+QSCAN:"NR5G",460,01,633984,223,-92,-12,28,1,69034E007,690E0F,273,78,28,4,1,"UNICOM","CH
d
N-UNICOM"
i
f
OK
n
o
3.3. Network Signal Strength
C
3.3.1. AT+CSQ Signal Quality Report
This command indicates the received signal strength <RSSI> and the channel bit error rate <ber>. This
Test Command returns values supported by MT. This Execution Command returns received signal
strength indication <RSSI> and channel bit error rate <ber> from MT.
AT+CSQ Signal Quality Report
Test Command Response
AT+CSQ=? +CSQ: (list of supported <RSSI>s),(list of supported <ber>s)
OK
Execution Command Response
AT+CSQ +CSQ: <RSSI>,<ber>
RG50xQ&RM5xxQ_Series_Network_Application_Note 36 / 136

5G Module Series
OK
If there is error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics /
Reference
3GPP TS 27.007
Parameter l
e
<RSSI> Integer type. Received signal strength indication.
t
0 -113 dBm or lecss
l
1 -111 dBm a
e
2–30 -109 dBm to -53 dBm
i
31 -51 dBm or greater
u t
99 Not known or not detectable
n
<ber> Q Integer type. Channel bit error rate (in percent).
0–7 As RxQual values in the tabele in 3GPP TS 45.008 subclause 8.2.4
99 Not known or not detectable
d
<err> Error codes. See Chapter 1 for details.
i
f
n
NOTE
o
This command only takes effect under WCDMA and LTE, and does not apply to NR5G.
C
Example
AT+CSQ=?
+CSQ: (0-31,99),(0-7,99)
OK
AT+CSQ
+CSQ: 28,99 //The current signal strength indication is 28 and channel bit error rate is not known
or not detectable.
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 37 / 136

                                                                5G Module Series

3.3.2. AT+QCSQ  Report Signal Quality
The command queries and reports the signal strength of the current service network. If MT is registered
to multiple networks in different service modes, you can query the signal strength of networks of each
mode. No matter whether MT is registered on a network or not, you can execute this command to query
the signal strength or allow MT to unsolicitedly report the detected signal strength if MT camps on the
network. If MT is not using any service network or the service mode is uncertain, "NOSERVICE" is
returned.
AT+QCSQ  Report Signal Quality
| Test Command    |     |     | Response                                |     |     |     |
| --------------- | --- | --- | --------------------------------------- | --- | --- | --- |
| AT+QCSQ=?       |     |     | +QCSQ: (list of supported <sy smode>s)  |     |     |     |
l

e
OK
| Read Command  |     |     | Response  | t   |     |     |
| ------------- | --- | --- | --------- | --- | --- | --- |
+QCcSQ: <enable>
AT+QCSQ?  l
a

e
OK
i
| Write Command     |     | uResponse  |     |     |     | t   |
| ----------------- | --- | ---------- | --- | --- | --- | --- |
| AT+QCSQ=<enable>  |     |            | OK  |     | n   |     |
Q
| Execution Command  |     |     | Response  |     |     |     |
| ------------------ | --- | --- | --------- | --- | --- | --- |
e
AT+QCSQ  +QCSQ: <sysmode>[,<value1>[,<value2>[,<value3>[,<val
ue4>]]]d]

i
OK
f
| Maximum Response Time  |     | n300 ms  |     |     |     |     |
| ---------------------- | --- | -------- | --- | --- | --- | --- |
This command takes effect immediately.
o
Characteristics
The configuration is saved automatically.
C
Parameter
<sysmode>     String type. Service mode in which MT unsolicitedly reports the signal strength.
|           |   "NOSERVICE"                            |     |   NOSERVICE mode  |     |     |     |
| --------- | ---------------------------------------- | --- | ----------------- | --- | --- | --- |
|           |   "WCDMA"                                |     |   WCDMA mode      |     |     |     |
|           |   "LTE"                                  |     |   LTE mode        |     |     |     |
|           |   "NR5G"                                 |     |   NR5G mode       |     |     |     |
| <valueX>  |   String type. "X" means 1, 2, 3 or 4.   |     |                   |     |     |     |
        <sysmode>   <value1>    <value2>    <value3>    <value4>
|       |   "NOSERVICE"  |     | Null     |   Null   |     Null   |     Null  |
| ----- | -------------- | --- | -------- | -------- | ---------- | --------- |
        "WCDMA"    wcdma_rssi    wcdma_rscp   wcdma_ecio   Null
        "LTE"      lte_rssi      lte_rsrp      lte_sinr      lte_rsrq
        "NR5G"     nr5g_rsrp    nr5g_sinr    nr5g_rsrq    Null
| <enable>  |   String type. Enable or disable URC report.  |     |     |     |     |     |
| --------- | --------------------------------------------- | --- | --- | --- | --- | --- |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                38 / 136

5G Module Series
0 Disable URC report
1 Enable URC report
NOTE
1. wcdma_rscp: An integer indicating the received signal code power, and it is available for WCDMA
mode.
2. wcdma_ecio: An integer indicating the downlink carrier-to-interference ratio, and it is available for
WCDMA mode.
3. lte_rsrp: An integer indicating the reference signal received power (RSRP), and it is available for
LTE mode.
4. lte_sinr: An integer indicating the signal to interference plus noise ratio (SINR), and it is available
l
for LTE mode. e
5. lte_rsrq: An integer indicating the reference signal received quality (RSRQ) in dB. and it is available
t
for LTE mode.
c
6. nr5g_rsrp: An integer indicating the reference signal received power (RSRP), and it is availalble for
a
NR5G mode. e
7. nr5g_sinr: An integer indicating the signal to interference plus noise ratio (SINR), and it is available
i
for NR5G mode. u t
n
8. nr5g_rsrq: An integer indicating the reference signal received quality (RSRQ) in dB. and it is
Q
available for NR5G mode.
e
9. URC reporting format as +QCSQ: <sysmode>[,<value1>[,<value2>[,<value3>[,<value4>]]]],
allows the MT to unsolicitedly report the currednt signal strength when the strength changes.
10. AT+QCSQ=<enable> controls the URC indication which is turned off by default (<enable>=0). If
i
<enable>=1, the MT can unsolicitedly report the current signal strength when the strength changes.
f
n
Example o
AT+QCSCQ //Query signal.
+QCSQ: "LTE",-52,-81,195,-10
OK
AT+QCSQ? //Query URC configuration.
+QCSQ: 0
OK
AT+QCSQ=? //Test command
+QCSQ: "NOSERVICE","WCDMA","LTE","NR5G"
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 39 / 136

                                                                5G Module Series

3.3.3. AT+QRSRP  Report RSRP
The command queries and reports the RSRP of the current service network.
AT+QRSRP  Report RSRP
| Test Command    |     |     | Response                                   |     |
| --------------- | --- | --- | ------------------------------------------ | --- |
| AT+QRSRP=?      |     |     | OK                                         |     |
| Read Command    |     |     | Response                                   |     |
| AT+QRSRP        |     |     | +QRSRP: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>  |     |
[…]

OK
l
| Maximum Response Time  |     |     | 300 ms  | e   |
| ---------------------- | --- | --- | ------- | --- |
| Characteristics        |     |     | /       | t   |
c
l
a
| Parameter  |     | e   |     |     |
| ---------- | --- | --- | --- | --- |
i
u type. PRX path RSRP value. Range: -140 to -44 dBtm.
| <PRX>  | Integer  |     |     |     |
| ------ | -------- | --- | --- | --- |
n
<DRX>  Integer type. DRX path RSRP value. Range: -140 to -44 dBm.
Q
<RX2>  Integer type. RX2 path RSRP value. Range: -140 to -44 dBm.
e
<RX3>  Integer type. RX3 path RSRP value. Range: -140 to -44 dBm.
<sysmode>  String type. Service mode in wdhich the MT reports the RSRP.
|       |   LTE   |   LTE mode  |     |     |
| ----- | ------- | ----------- | --- | --- |
i
|       |   NR5G  |   NR5G mode  |     |     |
| ----- | ------- | ------------ | --- | --- |
f
|     |     | n   |     |     |
| --- | --- | --- | --- | --- |

o
NOTE
1.  If theC queried <PRX>, <DRX>, <RX2> and <RX3> is -32768, it indicates that the RSRP value is
invalid.
2.  This command is strongly related to the RF link and is generally only used for customer reference
and cannot be used as a sensitivity test. In addition, it is best to use it when measuring the speed,
the results are more accurate.
Example
AT+QRSRP                  //Query RSRP.
+QRSRP: -101,-105,-105,-99,LTE

OK

RG50xQ&RM5xxQ_Series_Network_Application_Note                                40 / 136

5G Module Series
3.3.4. AT+QRSRQ Report RSRQ
The command queries and reports the RSRQ of the current service network.
AT+QRSRQ Report RSRQ
Test Command Response
AT+QRSRQ=? OK
Read Command Response
AT+QRSRQ +QRSRQ: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>
[…]
OK
l
Maximum Response Time 300 ms e
Characteristics /
t
c
l
a
e
Parameter
i
<PRX> Integer u type. PRX path RSRQ value. Range: -20 to -3 dB. t
n
<DRX> Integer type. DRX path RSRQ value. Range: -20 to -3 dB.
Q
<RX2> Integer type. RX2 path RSRQ value. Range: -20 to -3 dB.
e
<RX3> Integer type. RX3 path RSRQ value. Range: -20 to -3 dB.
<sysmode> String type. Service mode in wdhich the MT reports the RSRQ.
LTE LTE mode
i
NR5G NR5G mode
f
n
Example
o
AT+QRSRQ //Query RSRQ.
C
+QRSRQ: -16,-19,-19,-15,LTE
OK
NOTE
1. This command is only supported in LTE and NR5G.
2. If the queried <PRX>, <DRX>, <RX2> and <RX3> is -32768, it indicates that the RSRQ value is
invalid.
3. This command is strongly related to the RF link and is generally only used for customer reference
and cannot be used as a sensitivity test. In addition, it is best to use it when measuring the speed,
the results are more accurate.
RG50xQ&RM5xxQ_Series_Network_Application_Note 41 / 136

5G Module Series
3.3.5. AT+QSINR Report SINR
The command queries and reports the SINR of the current service network.
AT+QSINR Report SINR
Test Command Response
AT+QSINR=? OK
Read Command Response
AT+QSINR +QSINR: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>
[…]
OK
l
Maximum Response Time 300 ms e
Characteristics /
t
c
l
a
e
Parameter
i
<PRX> Integer typ u e. PRX path SINR value. Range: -20 to 30 dB in LTtE, -23 to 40 dB in NR5G.
n
<DRX> Integer type. DRX path SINR value. Range: -20 to 30 dB in LTE, -23 to 40 dB in NR5G.
Q
<RX2> Integer type. RX2 path SINR value. Range: -20 to 30 dB in LTE, -23 to 40 dB in NR5G.
e
<RX3> Integer type. RX3 path SINR value. Range: -20 to 30 dB in LTE, -23 to 40 dB in NR5G.
<sysmode> String type. Service mode in whicdh the MT reports the SINR.
LTE LTE mode
i
NR5G NR5G mode
f
n
NOTE
o
1. The invalid SINR value is -32768.
2. ThisC command is strongly related to the RF link and is generally only used for customer reference
and cannot be used as a sensitivity test. In addition, it is best to use it when measuring the speed,
the results are more accurate.
Example
AT+QSINR //Query SINR.
+QSINR: -3,-7,-1,-2,LTE
OK
3.3.6. AT+QRSSI Report RSSI
The command queries and reports the RSSI of the current service network.
RG50xQ&RM5xxQ_Series_Network_Application_Note 42 / 136

5G Module Series
AT+QRSSI Report RSSI
Test Command Response
AT+QRSSI=? OK
Execution Command Response
AT+QRSSI +QRSSI: <PRX>,<DRX>,<RX2>,<RX3>,<sysmode>
OK
Maximum Response Time 300 ms
Characteristics /
l
Parameter e
<PRX> Integer type. PRX path RSSI value.t Unit: dBm.
c
<DRX> Integer type. DRX path RSSI value. Unit: dBm. l
a
<RX2> Integer type. RX2 path RSSI value. Unit: dBm.
e
<RX3> Integer type. RX3 path RSSI value. Unit: dBm.
i
<sysmode> String tyupe without double quotes. Service mode in which thte MT reports the RSSI.
LTE LTE mode n
Q
NR5G NR5G mode
e
d
NOTE
i
f
If the queried <PRX>, <DRX>, <RX2> or <RX3> is -32768, it indicates that the RSSI value is invalid.
n
o
Example
C
AT+QRSSI //Query RSSI.
+QRSSI: -42,-56,-42,-40,NR5G
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 43 / 136

                                                                5G Module Series

3.4. General Commands
3.4.1. AT+CPOL  Preferred Operator List
This command edits and queries the list of preferred operators.
AT+CPOL  Preferred Operator List
| Test Command  |     |     | Response  |     |
| ------------- | --- | --- | --------- | --- |
AT+CPOL=?  +CPOL:  (list  of  supported  <index>s),(range  of  supported
<format>s)

  l
e
OK
| Read Command  |     |     | Response  |     |
| ------------- | --- | --- | --------- | --- |
t
Query the list of preferred operators:  +CcPOL: <index>,<format>,<oper>[,<GSM>,<GSM_comp
l
| AT+CPOL?  |     |     | act>,<UTRAN>,<E-UTRAN>,<NG-RAN>]  | a   |
| --------- | --- | --- | --------------------------------- | --- |
e
[…]
i

u t
OK
n
| Write CommaQnd  |     |     | Response  |     |
| --------------- | --- | --- | --------- | --- |
AT+CPOL=<index>[,<format>[,<ope Edit the list of preeferred operators:
| r>[<GSM>,<GSM_compact>,<UTRA |     |     | OK  |     |
| ---------------------------- | --- | --- | --- | --- |
d
| N>,<E-UTRAN>,<NG-RAN>]]]  |     |     | Or  |     |
| ------------------------- | --- | --- | --- | --- |
ERROR
i
f
nIf there is any error related to MT functionality:
+CME ERROR: <err>
o

If <index> is given but <oper> is omitted, this command
C
deletes the entry.
| Maximum Response Time  |     |     | 300 ms  |     |
| ---------------------- | --- | --- | ------- | --- |
| Characteristics        |     |     | /       |     |
| Reference              |     |     |         |     |
3GPP TS 27.007
Parameter
<index>         Integer type. The order number of operators in the (U)SIM preferred operator list.
| <format>  |     Integer type.  |                                     |     |     |
| --------- | ------------------ | ----------------------------------- | --- | --- |
|           |     0              |   Long format alphanumeric <oper>   |     |     |
|           |     1              |   Short format alphanumeric <oper>  |     |     |
|           |     2              |   Numeric <oper>                    |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                44 / 136

5G Module Series
<oper> Operator name. <format> indicates the format is alphanumeric or numeric (see
AT+COPS)
<GSM> Integer type. GSM access technology.
0 Access technology is not selected
1 Access technology is selected
<GSM_compact> Integer type. GSM compact access technology.
0 Access technology is not selected
1 Access technology is selected
<UTRAN> Integer type. UTRAN access technology.
0 Access technology is not selected
1 Access technology is selected
<E-UTRAN> Integer type. E-UTRAN access technology.
0 Access technology is not selected l
e
1 Access technology is selected
<NG-RAN> Integer type. NG-RAN access ttechnology.
0 Access tcechnology is not selected
l
1 Access technology is selected a
e
<err> Error codes. See Chapter 1 for details.
i
u t
n
Q
NOTE
e
The access technology selection parameters <GSM>, <GSM_compact>, <UTRAN> and <E-UTRAN>
are required for (U)SIM cards or UICC’s containindg PLMN selector with access technology.
i
f
3.4.2. AT+CPLS Select PnLMN Selector
This command selectos a PLMN selector with Access Technology list in the SIM card or active application
in the UICC (GSM or USIM), that is used by AT+CPOL.
C
AT+CPLS Select PLMN Selector
Test Command Response
AT+CPLS=? +CPLS: (list of supported <list>s)
OK
Write Command Response
AT+CPLS=<list> OK
Or
ERROR
If there is any error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time 300 ms
RG50xQ&RM5xxQ_Series_Network_Application_Note 45 / 136

                                                                5G Module Series

| Characteristics  |     | /   |     |     |     |
| ---------------- | --- | --- | --- | --- | --- |
Parameter
| <list>    | Integer type. Selection of Preferred PLMN List.  |     |     |     |     |
| --------- | ------------------------------------------------ | --- | --- | --- | --- |
            0  User-controlled PLMN selector with Access Technology EF . If it is not found
PLMNwAcT
in the SIM/UICC, the PLMN preferred list EF  is used. (This file is only available
PLMNsel
in SIM card or GSM application selected in UICC)
            1     Operator-controlled PLMN selector with Access Technology EF
OPLMNwAcT
            2     HPLMN selector with Access Technology EF HPLMNwAcT
| <err>    | Error codes. See Chapter 1 for details.  |     |     |     |     |
| -------- | ---------------------------------------- | --- | --- | --- | --- |
l

e
3.4.3. AT+CGDCONT  Define PDP Contexts
t
c
l
The command specifies PDP context parameters for a specific context <cid>. A special form of the Write
a
Command (AT+CGDCONT=<cid>) ecauses the values for context <cid> to become undefined. It is not
allowed to change the definition of an already activated context. This Read Commaind returns the current
u t
configurations for each defined PDP context.
n
Q
AT+CGDCONT  Define PDP Contexts
e
| Test Command  |     | Response  |     |     |     |
| ------------- | --- | --------- | --- | --- | --- |
d
| AT+CGDCONT=?  |     | +CGDCONT:                                  | (range  | of  | supported      |
| ------------- | --- | ------------------------------------------ | ------- | --- | -------------- |
|               |     | <cid>s),<PDP_type>,<APN>,<PDP_addr>,(list  |         |     | of  supported  |
i
<fd_comp>s),(list of supported <h_comp>s)[,(list of supported
n<IPv4AddrAlloc>s)[,(list of supported <request_type>s)[,(list of
supported <SSC_mode>s)[,(list of supported <S-NSSAI>s)[,(list
o
|     |     | of  supported            | <Pref_access_type>s)[,(list  |     | of  supported  |
| --- | --- | ------------------------ | ---------------------------- | --- | -------------- |
| C   |     | <Always-on_req>s)]]]]]]  |                              |     |                |

OK
| Read Command  |     | Response  |     |     |     |
| ------------- | --- | --------- | --- | --- | --- |
AT+CGDCONT?  +CGDCONT: <cid>,<PDP_type>,<APN>,<PDP_addr>,<d_co
mp>,<h_comp>[,<IPv4AddrAlloc>[,<request_type>,,,,,,,,[,<S
SC_mode>[,<S-NSSAI>[,<Pref_access_type>,,,[,<Always-on
_req>]]]]]]
[…]

OK
| Write Command                   |     | Response  |     |     |     |
| ------------------------------- | --- | --------- | --- | --- | --- |
| AT+CGDCONT=[<cid>[,<PDP_typ     |     | OK        |     |     |     |
| e>[,<APN>[,<PDP_addr>[,<d_com   |     | Or        |     |     |     |
| p>[,<h_comp>[,<IPv4AddrAlloc>[, |     | ERROR     |     |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                46 / 136

5G Module Series
<request_type>,,,,,,,,[,<SSC_mod
e>[,<S-NSSAI>[,<Pref_access_typ
e>,,,[,<Always-on_req>]]]]]]]]]]]]
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
Reference
3GPP TS 27.007
Parameter
l
<cid> Integer type. PDP context identifier. Specifies a particular PDP context definition. The
e
parameter is local to the TE-MT interface and is used in other PDP context-related
commands. The range of supportedt values (minimum value = 1) is returned by the test
c
form of the command. Range: 1–42. l
a
<PDP_type> String type. Packet data protocol type. Specifies the type of packet data protocol.
e
"IP" IPv4. Internet protocol (IETF STD 5)
i
"PPP" u Point to Point Protocol (IETF STD 51 [104]) t
"IPV6" Internet Protocol, version 6 (see RFCn 2460 [106])
Q
"IPV4V6" Virtual introduced to handle dual IP stack UE capability. (See 3GPP
e
TS24.301 [83])
<APN> String type. Access point namde, which is a logical name used to select the GGSN or
the external packet data network. If the value is null or omitted, then the subscription
value will be requested. i
f
<PDP_addr> String type. Identify the MT in the address space applicable to the PDP. If the value is
n
null or omitted, then a value may be provided by the TE during the PDP startup
proocedure or, failing that, a dynamic address will be requested. The allocated
address may be read using the AT+CGPADDR.
C
<d_comp> Integer type. Controls PDP data compression (applicable for SNDCP only) (see 3GPP
TS 44.065).
0 Off (Default if i is omitted)
2 V.42bis
<h_comp> Integer type. Controls PDP header compression (see 3GPP TS 44.065 and 3GPP TS
25.323).
0 Off
4 RFC3095
<IPv4AddrAlloc> Integer type. Controls how the MT/TA requests to get the IPv4 address
information.
0 IPv4 address allocation through NAS signaling
1 IPv4 address allocated through DHCP
<request_type> Integer type. Type of PDP context activation request for the PDP context.
0 PDP context is for new PDP context establishment or for handover from a
non-3GPP access network (how the MT decides whether the PDP context is
RG50xQ&RM5xxQ_Series_Network_Application_Note 47 / 136

                                                                5G Module Series

            for new PDP context establishment or for handover is implementation
|       |     |   specific).                                       |     |     |     |
| ----- | --- | -------------------------------------------------- | --- | --- | --- |
|       |     | 1   PDP context is for emergency bearer services.  |     |     |     |
<SSC_mode>  Integer type. Indicates the session and service continuity (SSC) mode for the PDU
|       |   session in 5GS, see 3GPP TS 23.501 [165].  |                                                |     |     |     |
| ----- | -------------------------------------------- | ---------------------------------------------- | --- | --- | --- |
|       |   0                                          | The PDU session is associated with SSC mode 1  |     |     |     |
|       |   1                                          | The PDU session is associated with SSC mode 2  |     |     |     |
<S-NSSAI>    String type of hexadecimal format. Dependent of the form, the string can be separated
        by dot(s) and semicolon(s). The S-NSSAI is associated with the PDU session for
        identifying a network slice in 5GS, see 3GPP TS 23.501 [165] and 3GPP TS 24.501
         [161]. For the format and the encoding of S-NSSAI, see also 3GPP TS 23.003 [7]. This
        parameter shall not be subject to conventional charac ter conversion as per AT+CSCS.
l
|       |   The has one of the forms:  |     |     |     |     |
| ----- | ---------------------------- | --- | --- | --- | --- |
e
        sst                only slice/service type (SST) is present
        sst;mapped_sst          tSST and mapped configured SST are present
        sst.sd          c    SST and slice differentiator (SD) are present
l
        sst.sd;mapped_sst        SST, SD and mapped configuread SST are present
e
        sst.sd;mapped_sst.mapped_sd   SST, SD, mapped configured SST and mapped
i
|       |     |   u    |       |   configured SD are pres | ent  |
| ----- | --- | ------ | ----- | ------------------------ | ---- |
t
<Pref_access_type>   Integer type. Preferred access type for thne PDU session in 5GS, see 3GPP
Q
|       |     |   TS 23.501 [165] and 3GPP TS 24.501 [161].  |     |     |     |
| ----- | --- | -------------------------------------------- | --- | --- | --- |
e
|       |     |   0  The preferred access type is 3GPP access      |     |     |     |
| ----- | --- | -------------------------------------------------- | --- | --- | --- |
|       |     |   1  The preferred access type is non-3GPP access  |     |     |     |
d
<Always-on_req>    Integer type. Whether the UE requests to establish the PDU session as an
            always-on PDU siession, see 3GPP TS 24.501 [161].
f
|       |     |   0  Always-on PDU session is not requested  |     |     |     |
| ----- | --- | -------------------------------------------- | --- | --- | --- |
n
|       |     |   1  Always-on PDU session is requested  |     |     |     |
| ----- | --- | ---------------------------------------- | --- | --- | --- |
o
Example
C
AT+CGDCONT=1,"IPV4V6","example",,,,,,
OK

3.4.4. AT+COPN  Read Operator Names
This command returns the list of the supported operator names from MT. Each operator code <numericn>
that has an alphanumeric equivalent <alphan> in the MT memory is returned.
AT+COPN  Read Operator Names
| Test Command       |     |     | Response                    |     |     |
| ------------------ | --- | --- | --------------------------- | --- | --- |
| AT+COPN=?          |     |     | OK                          |     |     |
| Execution Command  |     |     | Response                    |     |     |
| AT+COPN            |     |     | +COPN: <numeric1>,<alpha1>  |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                48 / 136

                                                                5G Module Series

[+COPN: <numeric2>,<alpha2>
[…]]

OK

If there is error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time  Depends on the number of operator names.
| Characteristics  |     | /   |
| ---------------- | --- | --- |
| Reference        |     |     |
3GPP TS 27.007
l
e
Parameter
t
c
l
<numericn>   String type. Operator name in numeric format (see AT+COPS).
a
| <alphan>  |   String type. Operator name in long alphanumeric format (see AT+COPS).  | e   |
| --------- | ------------------------------------------------------------------------ | --- |
i
| <err>         | Error codes. See Chapter 1 for details.  |     |
| ------------- | ---------------------------------------- | --- |
u t

n
Q
3.4.5. AT+CTZU  Automatic Time Zone Update
e
This command enables/disables automatic time zdone update via NITZ.
AT+CTZU  Automatic Time Zone Updiate
f
| Test Command  |     | Response  |
| ------------- | --- | --------- |
n
| AT+CTZU=?  |     | +CTZU: (list of supported <onoff>s)  |
| ---------- | --- | ------------------------------------ |
o

OK
C
| Write Command    |     | Response  |
| ---------------- | --- | --------- |
| AT+CTZU=<onoff>  |     | OK        |
Or
ERROR
| Read Command  |     | Response        |
| ------------- | --- | --------------- |
| AT+CTZU?      |     | +CTZU: <onoff>  |

OK
| Maximum Response Time  |     | 300 ms  |
| ---------------------- | --- | ------- |
The command takes effect immediately.
Characteristics
The configurations is saved automatically.
| Reference  |     |     |
| ---------- | --- | --- |
3GPP TS 27.007
RG50xQ&RM5xxQ_Series_Network_Application_Note                                49 / 136

5G Module Series
Parameter
<onoff> Integer type. Indicates the mode of automatic time zone update.
0 Disable automatic time zone update via NITZ
1 Enable automatic time zone update via NITZ
Example
AT+CTZU? //Read command.
+CTZU: 0
OK
AT+CTZU=? //Test command. l
e
+CTZU: (0,1)
t
OK c
l
AT+CTZU=1 //Enable automatic time zone update. a
e
OK
i
AT+CTZU? //Quuery the current configuration.
t
+CTZU: 1 n
Q
e
OK
d
3.4.6. AT+CTZR Time Zone Reporting
i
f
This command controls the repnorting of time zone change event. If reporting is enabled, MT returns the
unsolicited result code +CTZV: <tz> or +CTZE: <tz>,<dst>,<time> whenever the time zone is changed.
o
AT+CTZR Time Zone Reporting
C
Test Command Response
AT+CTZR=? +CTZR: (range of supported <reporting>s)
OK
Write Command Response
AT+CTZR=<reporting> OK
Or
ERROR
Read Command Response
AT+CTZR? +CTZR: <reporting>
OK
Maximum Response Time 300 ms
RG50xQ&RM5xxQ_Series_Network_Application_Note 50 / 136

5G Module Series
The command takes effect immediately.
Characteristics
The configurations is saved automatically.
Reference
3GPP TS 27.007
Parameter
<reporting> Integer type. Indicate the mode of time zone reporting.
0 Disable time zone reporting of changed event
1 Enable time zone reporting of changed event by unsolicited result code
+CTZV: <tz>
2 Enable extended time zone reporting by unsolicited result code
l
+CTZE: <tz>,<dst>,<time> e
<tz> String type. Sum of the local time zone (difference between the local time and GMT is
t
expressed in quarters of an hour) plus daylight saving time. The format is "±zz",
c
l
expressed as a fixed width, two-digit integer with the range -48 to +56. To maintain a
a
fixed width, numbeers in the range -9 to +9 are expressed with a leading zero, e.g. "-09",
"+00" and "+09". i
u t
<dst> Integer type. Whether <tz> includes daylight savings adjustment.
n
Q0 <tz> includes no adjustment for daylight saving time
1 <tz> includes +1 hour (equals 4 quearters in <tz>) adjustment for daylight saving
time
d
2 <tz> includes +2 hours (equals 8 quarters in <tz>) adjustment for daylight saving
time
i
<time> String type. Local timef. The format is "YYYY/MM/DD,hh:mm:ss", expressed as integers
representingn year (YYYY), month (MM), date (DD), hour (hh), minute (mm) and second
(ss). This parameter can be provided by the network when delivering time zone
o
information and will be presented in the unsolicited result code of extended time zone
C
reporting if provided by the network.
Example
AT+CTZR=2
OK
AT+CTZR?
+CTZR: 2
OK
+CTZE: "+32",0,"2018/03/23,06:51:13" //Extended time zone and local time reporting by URC.
RG50xQ&RM5xxQ_Series_Network_Application_Note 51 / 136

5G Module Series
3.4.7. AT+CCLK Clock
This command sets and queries the real time clock (RTC) of the MT. The current setting is retained until
the MT is totally disconnected from the power supply.
AT+CCLK Clock
Test Command Response
AT+CCLK=? OK
Read Command Response
AT+CCLK? +CCLK: <time>
OK
l
Write Command Response
e
AT+CCLK=<time> OK
t
c
If there is any error related to MT functionality: l
a
+CME ERROR: <err>
e
Maximum Response Time 300 ms i
u t
The command takes effect immediately.
n
Characteristics
Q The configuration is not saved.
e
Parameter d
i
<time > String type. The format is "yy/MM/dd,hh:mm:ss±zz", indicating year (two last digits),
f
month, day,n hour, minutes, seconds and time zone (indicates the difference, expressed
in quarters of an hour, between the local time and GMT; Range: -48 to +56). E.g. May
6tho
, 1994, 22:10:00 GMT+2 hours equals "94/05/06,22:10:00+08".
<err> Error codes. See Chapter 1 for details.
C
Example
AT+CCLK? //Query the local time.
+CCLK: "08/01/04,00:19:43+00"
OK
3.4.8. AT+QLTS Obtain the Latest Time Synchronized through Network
The Execution Command returns the latest time that has been synchronized through network.
AT+QLTS Obtain the Latest Time Synchronized through Network
Test Command Response
RG50xQ&RM5xxQ_Series_Network_Application_Note 52 / 136

5G Module Series
AT+QLTS=? +QLTS: (range of supported <mode>s)
OK
Execution Command Response
AT+QLTS +QLTS: <time>,<ds>
OK
Write Command Response
AT+QLTS=<mode> +QLTS: <time>,<ds>
OK
l
If there is any error:
e
ERROR
t
If thcere is any error related to MT functionality: l
+CME ERROR: <err> a
e
Maximum Response Time 300 ms
i
u t
Characteristics / n
Q
e
Parameter
d
<mode> Integer type. Query network time mode.
i
0 Query the latest timef that has been synchronized through network
1 Query then current GMT time calculated from the latest time that has been
synchronized through network
o
2 Query the current LOCAL time calculated from the latest time that has been
Csynchronized through network
<time> Format is "yy/MM/dd,hh:mm:ss±zz", in which characters indicate year (two last digits),
month, day, hour, minutes, seconds and time zone (indicates the difference, expressed in
quarters of an hour, between the local time and GMT; range: -48 to +48). E.g. 6th of May
2004, 22:10:00 GMT+2 hours equals "04/05/06,22:10:00+08".
<ds> Integer type. Daylight saving time.
0 No adjustment
1 Plus one hour
2 Plus two hours
<err> Error codes. See Chapter 1 for details.
NOTE
If the time has not been synchronized through network, the command returns +QLTS: "".
RG50xQ&RM5xxQ_Series_Network_Application_Note 53 / 136

5G Module Series
Example
AT+QLTS=? //Query supported network time modes.
+QLTS: (0-2)
OK
AT+QLTS //Query the latest time synchronized through network.
+QLTS: "2017/01/13,03:40:48+32,0"
OK
AT+QLTS=0 //Query the latest time synchronized through network. It offers the same
function as Execution Command AT+QLTS.
+QLTS: "2017/01/13,03:40:48+32,0"
l
e
OK
t
AT+QLTS=1 //Query the current GMT time calculated from the latest time that has been
c
synchronized through network. l
a
+QLTS: "2017/01/13,03:41:22+32,0e"
i
OK u t
n
AT+QLTS=2 //Query the current LOCAL time calculated from the latest time that has been
Q
synchronized through network.
e
+QLTS: "2017/01/13,11:41:23+32,0"
d
OK
i
f
n
3.4.9. AT+QSPN Query Service Provider Name
o
This command queries the service provider name.
C
AT+QSPN Query Service Provider Name
Test Command Response
AT+QSPN=? OK
Execution Command Response
AT+QSPN +QSPN: <FNN>,<SNN>,<SPN>,<alphabet>,<RPLMN>
OK
Or
ERROR
Maximum Response Time 300 ms
Characteristics /
RG50xQ&RM5xxQ_Series_Network_Application_Note 54 / 136

5G Module Series
Parameter
<FNN> String type. Full name of network.
<SNN> String type. Shortened name of network.
<SPN> String type. Service provider name.
<alphabet> Integer type. Alphabet of full and shortened network name.
0 GSM 7-bit default alphabet
1 UCS2
<RPLMN> String type. Registered PLMN.
NOTE
l
1. If <alphabet> is 0, <FNN> and <SNN> are shown in GSM 7-bit default alphabet string.
e
2. If <alphabet> is 1, <FNN> and <SNN> are shown in UCS2 hexadecimal string.
3. While network is not registered, AT+QSPN will retturn OK.
c
l
a
e
Example
i
u t
AT+QSPN //Query the service provider name.
n
+QSPN: "CHQN-UNICOM","UNICOM","",0,"46001"
e
OK
d
i
3.4.10. AT+QNETRC Get the Net Reject Cause
f
n
This command gets the net reject cause. This Write Command sets whether to present URC and controls
the presentation of UoRC +QNETRC: "emm_cause",<emm_reject_cause> when <mode> & 0x01 = 1
and the module received a rejection code issued by the network during LTE network registration, or URC
C
+QNETRC: "esm_cause",<esm_reject_cause> when <mode> & 0x02 = 2 and the module received a
rejection code issued by the network during LTE session management process, or URC +QNETRC:
"5gmm_cause",<5gmm_reject_cause> when <mode> & 0x4 = 4 and the module received a rejection
code issued by the network during 5G network registration.
AT+QNETRC Get the Net Reject Cause
Read Command Response
AT+QNETRC? +QNETRC: "emm_cause",<emm_reject_cause>
+QNETRC: "esm_cause",<esm_reject_cause>
+QNETRC: "5gmm_cause",<5gmm_reject_cause>
OK
Write Command Response
AT+QNETRC=<mode> OK
Or
RG50xQ&RM5xxQ_Series_Network_Application_Note 55 / 136

                                                                5G Module Series

ERROR
| Execution Command  |     |     | Response         |     |     |
| ------------------ | --- | --- | ---------------- | --- | --- |
| AT+QNETRC          |     |     | +QNETRC: <mode>  |     |     |

OK
| Characteristics  |     |     | /   |     |     |
| ---------------- | --- | --- | --- | --- | --- |
Parameter
<mode>                    Integer type. Determines the output type of URC sentences by ORed.

|     |     |     0  |   No URC report  |     |     |
| --- | --- | ------ | ---------------- | --- | --- |
l
e
|     |     |     1  |   EMM URC  |     |     |
| --- | --- | ------ | ---------- | --- | --- |
|     |     |     2  |   ESM URC  |     |     |
t
|     |     |     4  |   5GMM URC  |     |     |
| --- | --- | ------ | ----------- | --- | --- |
c
l
| <emm_reject_cause>      |     | Integer type. EMM reject cause.   |     |     |     |
| ----------------------- | --- | --------------------------------- | --- | --- | --- |
a
|     |       |     0  | e  No cause            |     |     |
| --- | ----- | ------ | ---------------------- | --- | --- |
|     |       |     2  |   IMSI unknown in HSS  |     | i   |
|     |       | u      |                        | t   |     |
|     |       |     3  |   Illegal UE           |     |     |
n
|     |   Q    |     5  |   IMEI not accepted         |     |     |
| --- | ------ | ------ | --------------------------- | --- | --- |
|     |        |     6  |   Illegal ME                | e   |     |
|     |        |     7  |   EPS services not allowed  |     |     |
d
              8    EPS services and non-EPS services not allowed
              9    UE identity cannot be derived by the network
i
|     |       |     10   |   fImplicitly detached       |     |     |
| --- | ----- | -------- | ---------------------------- | --- | --- |
|     |       |     n11  |   PLMN not allowed           |     |     |
|     |       |     12   |   Tracking Area not allowed  |     |     |
o
              13    Roaming not allowed in this tracking area
|     | C       |     14  |   EPS services not allowed in this PLMN  |     |     |
| --- | ------- | ------- | ---------------------------------------- | --- | --- |
|     |         |     15  |   No Suitable Cells in tracking area     |     |     |
|     |         |     16  |   MSC temporarily not reachable          |     |     |
|     |         |     17  |   Network failure                        |     |     |
|     |         |     18  |   CS domain not available                |     |     |
|     |         |     19  |   ESM failure                            |     |     |
|     |         |     20  |   MAC failure                            |     |     |
|     |         |     21  |   Synch failure                          |     |     |
|     |         |     22  |   Congestion                             |     |     |
|     |         |     23  |   UE security capabilities mismatch      |     |     |
|     |         |     24  |   Security mode rejected, unspecified    |     |     |
|     |         |     25  |   Not authorized for this CSG            |     |     |
|     |         |     26  |   Non-EPS authentication unacceptable    |     |     |
|     |         |     31  |   Redirection to 5GCN required           |     |     |
              35    Requested service option not authorized in this PLMN
|     |       |     39  |   CS service temporarily not available  |     |     |
| --- | ----- | ------- | --------------------------------------- | --- | --- |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                56 / 136

                                                                5G Module Series

|       |     |     40  |   No EPS bearer context activated  |     |     |
| ----- | --- | ------- | ---------------------------------- | --- | --- |
|       |     |     42  |   Severe network failure           |     |     |
|       |     |     95  |   Semantically incorrect message   |     |     |
|       |     |     96  |   Invalid mandatory information    |     |     |
              97    Message type non-existent or not implemented
              98    Message type not compatible with the protocol state
              99    Information element non-existent or not implemented
|       |     |     100   | Conditional IE error  |     |     |
| ----- | --- | --------- | --------------------- | --- | --- |
              101   Message not compatible with the protocol state
|                          |     |     111                          | Protocol error, unspecified  |     |     |
| ------------------------ | --- | -------------------------------- | ---------------------------- | --- | --- |
| <esm_reject_cause>       |     | Integer type. ESM reject cause.  |                              |     |     |
|                          |     |     0                            |   No cause                   |     |     |

Operator Determined Barrinlg
|       |     |     8  |     |     |     |
| ----- | --- | ------ | --- | --- | --- |
e
|       |     |     26  |   Insufficient resources  |     |     |
| ----- | --- | ------- | ------------------------- | --- | --- |
|       |     |     27  |   Missing or unknown APN  | t   |     |
|       |     |     28  |   Unkcnown PDN type       |     |     |
l
|       |     |     29  |   User authentication failed  |     | a   |
| ----- | --- | ------- | ----------------------------- | --- | --- |
e
              30    Request rejected by Serving GW or PDN GW
i
|       |     |   u  31  |   Request rejected, unspecified  |     |     |
| ----- | --- | -------- | -------------------------------- | --- | --- |
t
|       |     |     32  |   Service option not supportend  |     |     |
| ----- | --- | ------- | -------------------------------- | --- | --- |
Q
|       |     |     33  |   Requested service option not subscribed  |     |     |
| ----- | --- | ------- | ------------------------------------------ | --- | --- |
e
|       |     |     34  |   Service option temporarily out of order  |     |     |
| ----- | --- | ------- | ------------------------------------------ | --- | --- |
|       |     |     35  |   PTI already in use                       |     |     |
d
|       |     |     36  |   Regular deactivation   |     |     |
| ----- | --- | ------- | ------------------------ | --- | --- |
|       |     |     37  |   EPSi QoS not accepted  |     |     |
f
|       |     |     38  |   Network failure  |     |     |
| ----- | --- | ------- | ------------------ | --- | --- |
n
|       |     |     39  |   Reactivation requested               |     |     |
| ----- | --- | ------- | -------------------------------------- | --- | --- |
|       |     |     41  |   Semantic error in the TFT operation  |     |     |
o
|       |     |     42  |   Syntactical error in the TFT operation  |     |     |
| ----- | --- | ------- | ----------------------------------------- | --- | --- |
C
|       |     |     43  |   Invalid EPS bearer identity             |     |     |
| ----- | --- | ------- | ----------------------------------------- | --- | --- |
|       |     |     44  |   Semantic errors in packet filter(s)     |     |     |
|       |     |     45  |   Syntactical errors in packet filter(s)  |     |     |
|       |     |     46  |   Unused (see NOTE 2)                     |     |     |
|       |     |     47  |   PTI mismatch                            |     |     |
|       |     |     49  |   Last PDN disconnection not allowed      |     |     |
|       |     |     50  |   PDN type IPv4 only allowed              |     |     |
|       |     |     51  |   PDN type IPv6 only allowed              |     |     |
|       |     |     52  |   Single address bearers only allowed     |     |     |
|       |     |     53  |   ESM information not received            |     |     |
|       |     |     54  |   PDN connection does not exist           |     |     |
              55    Multiple PDN connections for a given APN not allowed
              56    Collision with network initiated request
|       |     |     57  |   PDN type IPv4v6 only allowed  |     |     |
| ----- | --- | ------- | ------------------------------- | --- | --- |
|       |     |     58  |   PDN type non IP only allowed  |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                57 / 136

                                                                5G Module Series

|       |       |   59    | Unsupported QCI value                  |     |     |
| ----- | ----- | ------- | -------------------------------------- | --- | --- |
|       |       |   60    | Bearer handling not supported          |     |     |
|       |       |   61    | PDN type Ethernet only allowed         |     |     |
|       |       |   65    | Maximum number of EPS bearers reached  |     |     |
              66    Requested APN not supported in current RAT and PLMN
|       |       |         | combination                     |     |     |
| ----- | ----- | ------- | ------------------------------- | --- | --- |
|       |       |   81    | Invalid PTI value               |     |     |
|       |       |   95    | Semantically incorrect message  |     |     |
|       |       |   96    | Invalid mandatory information   |     |     |
              97    Message type non-existent or not implemented
              98    Message type not compatible with the protocol state
              99    Information element non-existe nt or not implemented
l
|       |       |   100   | Conditional IE error  |     |     |
| ----- | ----- | ------- | --------------------- | --- | --- |
e
              101   Message not compatible with the protocol state
|       |       |   111   | Protocol error, unspecified  | t   |     |
| ----- | ----- | ------- | ---------------------------- | --- | --- |
              112   APNc restriction value incompatible with active EPS bearer
l
|       |       |       | context  |     | a   |
| ----- | ----- | ----- | -------- | --- | --- |
e
              113   Multiple accesses to a PDN connection not allowed
i
| <5gmm_reject_cause> u    |     | Integer type. 5GMM reject cause.  |     |     |     |
| ------------------------ | --- | --------------------------------- | --- | --- | --- |
t
|       |       |   0    | No cause  |     |     |
| ----- | ----- | ------ | --------- | --- | --- |
n
Q
|       |     |   3    | Illegal UE  |     |     |
| ----- | --- | ------ | ----------- | --- | --- |
e
|       |       |   5    | PEI not accepted  |     |     |
| ----- | ----- | ------ | ----------------- | --- | --- |
|       |       |   6    | Illegal ME        |     |     |
d
|       |       |   7    | 5GS services not allowed  |     |     |
| ----- | ----- | ------ | ------------------------- | --- | --- |
              9    UE iidentity cannot be derived by the network
f
|       |       |   10    | Implicitly de-registered  |     |     |
| ----- | ----- | ------- | ------------------------- | --- | --- |
n
|       |       |   11    | PLMN not allowed           |     |     |
| ----- | ----- | ------- | -------------------------- | --- | --- |
|       |       |   12    | Tracking area not allowed  |     |     |
o
              13    Roaming not allowed in this tracking area
C
|       |       |   15    | No suitable cells in tracking area      |     |     |
| ----- | ----- | ------- | --------------------------------------- | --- | --- |
|       |       |   20    | MAC failure                             |     |     |
|       |       |   21    | Synch failure                           |     |     |
|       |       |   22    | Congestion                              |     |     |
|       |       |   23    | UE security capabilities mismatch       |     |     |
|       |       |   24    | Security mode rejected, unspecified     |     |     |
|       |       |   26    | Non-5G authentication unacceptable      |     |     |
|       |       |   27    | N1 mode not allowed                     |     |     |
|       |       |   28    | Restricted service area                 |     |     |
|       |       |   31    | Redirection to EPC required             |     |     |
|       |       |   43    | LADN not available                      |     |     |
|       |       |   62    | No network slices available             |     |     |
|       |       |   65    | Maximum number of PDU sessions reached  |     |     |
              67    Insufficient resources for specific slice and DNN
              69    Insufficient resources for specific slice
RG50xQ&RM5xxQ_Series_Network_Application_Note                                58 / 136

                                                                5G Module Series

|       |       |   71    | ngKSI already in use                 |     |
| ----- | ----- | ------- | ------------------------------------ | --- |
|       |       |   72    | Non-3GPP access to 5GCN not allowed  |     |
|       |       |   73    | Serving network not authorized       |     |
              74    Temporarily not authorized for this SNPN
              75    Permanently not authorized for this SNPN
              76    Not authorized for this CAG or authorized for CAG cells only
|       |       |   77    | Wireline access area not allowed  |     |
| ----- | ----- | ------- | --------------------------------- | --- |
              78    PLMN not allowed to operate at the present UE location
|       |       |   79    | UAS services not allowed   |     |
| ----- | ----- | ------- | -------------------------- | --- |
|       |       |   90    | Payload was not forwarded  |     |
              91    DNN not supported or not subscribed in the slice
              92    Insufficient user-plane resource s for the PDU session
Semantically incorrect meslsage
|       |       |   95    |     |     |
| ----- | ----- | ------- | --- | --- |
e
|       |       |   96    | Invalid mandatory information                 |     |
| ----- | ----- | ------- | --------------------------------------------- | --- |
|       |       |   97    | Message type non-existent or not implemented  | t   |
              98    Mescsage type not compatible with the protocol state
l
              99    Information element non-existent or not impleamented
e
|       |       |   100   | Conditional IE error  |     |
| ----- | ----- | ------- | --------------------- | --- |
i
            u  101   Message not compatible with the pro tocol state
t
|       |       |   111   | Protocol error, unspecified n |     |
| ----- | ----- | ------- | ----------------------------- | --- |
Q
e
Example
d
AT+QNETRC=7
| OK  |     |     | i   |     |
| --- | --- | --- | --- | --- |
f
AT+QNETRC
n
+QNETRC: 7
|     | o   |     |     |     |
| --- | --- | --- | --- | --- |
OK
AT+QNECTRC?
+QNETRC: "emm_cause",7
+QNETRC: "esm_cause",0
+QNETRC: "5gmm_cause",0

OK

3.5. Packet Domain Commands
3.5.1. AT+CGACT  Activate/Deactive PDP Contexts
This command activates or deactivates the specified PDP context(s). If any PDP context is already in the
requested state, the state for that context remains unchanged. If the requested state for any specified
context cannot be achieved, an ERROR or +CME ERROR is returned. Extended error responses are
RG50xQ&RM5xxQ_Series_Network_Application_Note                                59 / 136

5G Module Series
enabled by AT+CMEE. If the MT is not PS attached when the activation form of the command is executed,
the MT first performs a PS attach and then attempts to activate the specified contexts. If the attach fails
then the MT responds with ERROR or, if extended error responses are enabled, with the appropriate
failure-to-attach error message.
For EPS, if an attempt is made to disconnect the last PDN connection, then the MT responds with ERROR,
or, if extended error responses are enabled, it responds with +CME ERROR.
For EPS, the activation request for an EPS bearer resource will be answered by the network by either an
EPS dedicated bearer activation or EPS bearer modification request. The request must be accepted by
the MT before the PDP context can be set in to established state.
For 5GS, the command is used to request or delete the specified QoSl flow. The request for a specific QoS
e
flow will be answered by the network by a PDU session establishment accept message or a PDU session
modification command message. The PDU session establishment accept message or a PDU session
t
modification command message must be acccepted by the MT before the QoS flow can be set to active
l
state. a
e
AT+CGACT Activate/Deactive PDP Context i
u t
Test Command Response n
Q
AT+CGACT=? +CGACT: (list of supported <state>s)
e
OK
d
Read Command Response
AT+CGACT? +CGiACT: <cid>,<state>
f
[…]
n
o OK
Write Command Response
C
AT+CGACT=[<state>[,<cid>[,<cid>[, OK
…]]]] Or
ERROR
If there is any error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time 150 s, determined by network
Characteristics /
Reference
3GPP TS 27.007
RG50xQ&RM5xxQ_Series_Network_Application_Note 60 / 136

5G Module Series
Parameter
<state> Integer type. Indicate the state of PDP context activation.
0 Deactivated
1 Activated
<cid> Integer type. Specify a particular PDP context definition (see AT+CGDCONT).
<err> Error codes. See Chapter 1 for details.
Example
AT+CGACT=1,3,4
OK
l
e
3.5.2. AT+CGATT Attachment or Detachment of PS
t
c
This command attaches the MT to, or detaches the MT from, the Packet Domain service. If thle MT is
a
already in the requested state, the coemmand is ignored and the OK response is returned. If the requested
state cannot be achieved, an ERROR or +CME ERROR is returned. Extended error riesponses are enabled
by AT+CMEE. u t
n
Q
AT+CGATT Attachment or Detachment of PS
e
Test Command Response
AT+CGATT=? +CGATdT: (list of supported <state>s)
i
OK
f
Read Command nResponse
AT+CGATT? +CGACT: <state>
o
OK
C
Write Command Response
AT+CGATT=<state> OK
Or
ERROR
If there is any error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time 30 s
Characteristics /
Reference
3GPP TS 27.007
RG50xQ&RM5xxQ_Series_Network_Application_Note 61 / 136

5G Module Series
Parameter
<state> Integer type. Indicate the state of PS attachment.
0 Detached
1 Attached
<err> Error codes. See Chapter 1 for details.
Example
AT+CGATT=0
OK
l
3.5.3. AT+CGPADDR Show PDP Addresses e
t
This command returns a list of PDP addresses for the specified context identifiers. If no <cid> is specified,
c
the addresses for all defined contexts are returned. l
a
e
AT+CGPADDR Show PDP Addresses
i
u t
Test Command Response
n
AT+CGPADDR=? +CGPADDR: (list of supported <cid>s)
Q
e
OK
Execution/Write Command Respondse
AT+CGPADDR[=<cid>[,<cid>[,…]]] +CGPADDR: <cid>,<PDP_addr>
i
f[…]
n
OK
o
If there is any error:
C
ERROR
Maximum Response Time 300 ms
Characteristics /
Reference
3GPP TS 27.007
Parameter
<cid> Integer type. Specify a particular PDP context definition (see AT+CGDCONT).
<PDP_addr> String type. Identifies the MT in the address space applicable to the PDP. The address
may be static or dynamic. For a static address, it will be the one set by the
AT+CGDCONT when the context was defined. For a dynamic address it is the one
assigned during the last PDP context activation that used the context definition
RG50xQ&RM5xxQ_Series_Network_Application_Note 62 / 136

                                                                5G Module Series

        referred to by <cid>. <PDP_addr> is omitted if no address is available.
Example
| AT+CGDCONT=1,"IP","UNINET"  |     |     |     | //Define a PDP context.  |
| --------------------------- | --- | --- | --- | ------------------------ |
OK
| AT+CGACT=1,1  |     |     |       | //Activated the PDP.  |
| ------------- | --- | --- | ----- | --------------------- |
OK
| AT+CGPADDR=1   |     |     |       | //Show the PDP address.  |
| -------------- | --- | --- | ----- | ------------------------ |
+CGPADDR: 1,"10.76.51.180"

OK
l

e
3.5.4. AT+CGEQOSRDP  Read EPS Quality of Service Dynamic Parameters
t
c
This command command shows the network assigned EPS QoS parameters for an EPS bearer relsource.
a
In UMTS/GPRS mode, it will hold a meapping function to the UMTS/GPRS QoS parameters.
i
AT+CGEQOSRDP  Reuad EPS Quality of Service Dynamic Paramteters
n
| Write Command    |     |     |     | Response  |
| ---------------- | --- | --- | --- | --------- |
Q
AT+CGEQOSRDP[=<cid>]  +CGEQOSRDP: <cid>,<QCI>,[<DL_GBR>,<UL_GBR>],[<
e
DL_MBR>,<UL_MBR>]
d
[...]]

i
fOK
n
| Maximum Response Time  |     |     |     | 300 ms  |
| ---------------------- | --- | --- | --- | ------- |
o
| Characteristics  |     |     |     | /   |
| ---------------- | --- | --- | --- | --- |
ReferencCe

3GPP TS 27.007
Parameter
<cid>       It is dynamically allocated by DS and associated to the dedicated bearer, the value
|       |     |  starts from 100.  |     |     |
| ----- | --- | ------------------ | --- | --- |
<QCI>       Integer type. Specifies a class of EPS QoS (see 3GPP TS 23.203 and 3GPP
|        |     |  TS24.301).                                 |                                            |     |
| ------ | --- | ------------------------------------------- | ------------------------------------------ | --- |
|        |     |  0                                          |   QCI is selected by network               |     |
|        |     |  [1 – 4]                                    | Value range for GBR Traffic Flows          |     |
|        |     |  65, 66, 67  Values for GBR Traffic Flows   |                                            |     |
|        |     |  [71 – 76]                                  | Value range for GBR Traffic Flows          |     |
|        |     |  [82 – 85]                                  | Value range for GBR Traffic Flows          |     |
|        |     |  [5 – 9]                                    |   Value range for non- GBR Traffic Flows   |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                63 / 136

                                                                5G Module Series

|       |    69, 70, 79                                          | Values for non-GBR Traffic Flows   |     |     |     |     |
| ----- | ------------------------------------------------------ | ---------------------------------- | --- | --- | --- | --- |
|       |    [128 – 254] Value range for Operator-specific QCIs  |                                    |     |     |     |     |
<DL_GBR>     Integer type. Indicates DL GBR in case of GBR QCI. The value is in kbit/s. This
|       |      parameter is omitted for a non-GBR QCI   |     |     |     |     |     |
| ----- | --------------------------------------------- | --- | --- | --- | --- | --- |
<UL_GBR>     Integer type. Indicates UL GBR in case of GBR QCI. The value is in kbit/s. This
|       |    parameter is omitted for a non-GBR QCI   |     |     |     |     |     |
| ----- | ------------------------------------------- | --- | --- | --- | --- | --- |
<DL_MBR>     Integer type. Indicates DL MBR in case of GBR QCI. The value is in kbit/s. This
|       |    parameter is omitted for a non-GBR QCI   |     |     |     |     |     |
| ----- | ------------------------------------------- | --- | --- | --- | --- | --- |
<UL_MBR>     Integer type. Indicates UL MBR in case of GBR QCI. The value is in kbit/s. This
|       |    parameter is omitted for a non-GBR QCI  |     |     |     |     |     |
| ----- | ------------------------------------------ | --- | --- | --- | --- | --- |

Example
l
e
| AT+CGEQOSRDP                           |     |     |     |     |     |     |
| -------------------------------------- | --- | --- | --- | --- | --- | --- |
| +CGEQOSRDP: 108,1,20,20,300000,100000  |     |     | t   |     |     |     |
+CGEQOSRDP: 109,9,0,0,0,0   c
l
|     |     |     |     |     | a   |     |
| --- | --- | --- | --- | --- | --- | --- |
e
OK
i
AT+CGEQOSRDP=109
|                            | u   |     |     | t   |     |     |
| -------------------------- | --- | --- | --- | --- | --- | --- |
| +CGEQOSRDP: 109,9,0,0,0,0  |     |     | n   |     |     |     |
Q

e
OK

d
3.5.5. AT+CGTFTRDP  Read Traffic Flow Template Dynamic Parameters
i
f
This command shows the netwnork assigned Traffic Flow Template for an EPS bearer resource.
AT+CGTFTRDP  oRead Traffic Flow Template Dynamic Parameters
Write Command    Response
C
AT+CGTFTRDP[=<cid>]  +CGTFTRDP: <cid>,<packet filter identifier>,<evaluation
|     |     | precedence       | index>,<remote  | address  | and      | subnet  |
| --- | --- | ---------------- | --------------- | -------- | -------- | ------- |
|     |     | mask>,<protocol  | number          | (ipv4)   | /  next  | header  |
(ipv6)>,<local port range>, <remote port range>,<ipsec
|     |     | security  parameter  | index  | (spi)>,<type  | of  service  | (tos)  |
| --- | --- | -------------------- | ------ | ------------- | ------------ | ------ |
(ipv4) and mask / traffic class (ipv6) and mask>,<flow label
(ipv6)>,<direction>,<NW packet filter Identifier>
[...]]

OK
Maximum Response Time  300 ms
Characteristics  /
Reference
RG50xQ&RM5xxQ_Series_Network_Application_Note                                64 / 136

                                                                5G Module Series

3GPP TS 27.007
Parameter
<cid>                 It is dynamically allocated by DS and associated to the dedicated
|       |       |       | bearer, the value starts from 100  |
| ----- | ----- | ----- | ---------------------------------- |
<packet filter identifier>        Integer type. The value range is from 1 to 16.
<evaluation precedence index>     Integer type. The value range is from 0 to 255.
<remote address and subnet mask>  String type. The string is given as dot-separated numeric (0-255)
|       |       |       | parameters on the form:  |
| ----- | ----- | ----- | ------------------------ |
                  "a1.a2.a3.a4.m1.m2.m3.m4" fo r IPv4 or
"a1.a2.a3.a4.a5.a6.a7.a8.al9.a10.a11.a12.a13.a14.a15.a16.
|       |       |       |     |
| ----- | ----- | ----- | --- |
e
                  m1.m2.m3.m4.m5.m6.m7.m8.m9.m10.m11.m12.m13.m14.
|       |       |       | m15.m16" fotr IPv6.  |
| ----- | ----- | ----- | -------------------- |
                  Whecn +CGPIAF is supported, its settings can influence the
l
                  format of this parameter returned with the exeacute form of
e
|       |       |       | +CGTFTRDP.  |
| ----- | ----- | ----- | ----------- |
i
<protocol number (ipv4u) / next header (ipv6)> Integer type. The value rang t e is from 0 to 255.
<local port range>          String type. The string is ginven as dot-separated numeric (0-
Q
|       |       |       | 65535) parameters on the form "f.t".  |
| ----- | ----- | ----- | ------------------------------------- |
e
<remote port range>         string type. The string is given as dot-separated numeric (0-
|       |       |       | 65535) parameters on the form "f.t".  |
| ----- | ----- | ----- | ------------------------------------- |
d
<ipsec security parameter index (spi)> Numeric value in hexadecimal format. The value range is
|       |       |       | fromi 00000000 to FFFFFFFF.  |
| ----- | ----- | ----- | ---------------------------- |
f
<type of service (tos) (ipv4) and mask / traffic class (ipv6) and mask>
n
                  String type. The string is given as dot-separated numeric (0-255)
|       |     o  |       | parameters on the form "t.m".  |
| ----- | ------ | ----- | ------------------------------ |
<flow label (ipv6)>          Numeric value in hexadecimal format. The value range is from
C
|     |       |       | 00000 to FFFFF. Valid for IPv6 only.  |
| --- | ----- | ----- | ------------------------------------- |
<direction>              Integer type. Specifies the transmission direction in which the
|       |       |       | Packet Filter shall be applied.  |
| ----- | ----- | ----- | -------------------------------- |
                  0  Pre Release 7 TFT Filter (see 3GPP TS 24.008 [8], table
|       |       |       |   10.5.162)   |
| ----- | ----- | ----- | ------------- |
|       |       |       | 1   Uplink    |
|       |       |       | 2   Downlink  |
                  3   Bidirectional (Used for Uplink and Downlink)
<NW packet filter Identifier>      Integer type. The value range is from 1 to 16. In EPS the value
                  is assigned by the network when established.
Example
| AT+CGTFTRDP  |     |     |     |
| ------------ | --- | --- | --- |
+CGTFTRDP: 108,0,191,"1.1.1.1.0.0.0.0",1,0.0,0.0,0,0.0,0,3,0
RG50xQ&RM5xxQ_Series_Network_Application_Note                                65 / 136

5G Module Series
+CGTFTRDP: 109,0,191,"3.3.3.3.0.0.0.0",1,0.0,0.0,0,0.0,0,3,0
OK
AT+CGTFTRDP=109
+CGTFTRDP: 109,0,191,"3.3.3.3.0.0.0.0",1,0.0,0.0,0,0.0,0,3,0
OK
3.5.6. AT+QGPAPN Query Activated APNs
This command queries activated APNs.
l
AT+QGPAPN Query Activated APNs
e
Test Command Response
t
AT+QGPAPN=? +QGPAPN: <cid>,<APN_name>
c
l
a
eOK
Write Command Response i
u t
AT+QGPAPN[=<mode>] If the optional parameter is omitted or is 0:
n
+QGPAPN: <cid>,<APN_name>
Q
[…]
e
OK d
i
fIf the optional parameter is specified to 1:
n+QGPAPN: <cid>,<APN_name>[,<PDP_addr>]
[…]
o
OK
C
Maximum Response Time 300 ms
Characteristics /
Parameter
<mode> Integer type. Hide or show the extension parameter options
0 Hide extension parameters
1 Show extension parameters
<cid> Integer type. PDP context identifier. See AT+CGDCONT for more information.
<APN_name> String type. The name of the activated APN.
<PDP_addr> String type. Identifies the MT in the address space applicable to the PDP. The address
may be static or dynamic. For a static address, it will be the one set by AT+CGDCONT
when the context was defined. For a dynamic address it is the one assigned during the
RG50xQ&RM5xxQ_Series_Network_Application_Note 66 / 136

5G Module Series
last PDP context activation that used the context definition referred to by <cid>.
<PDP_addr> is omitted if no address is available.
Example
AT+QGPAPN
+QGPAPN: 1,"cmnet"
+QGPAPN: 2,"ims"
+QGPAPN: 3,""
+QGPAPN: 4,"cmwap"
+QGPAPN: 5,""
OK l
e
AT+QGPAPN=1
+QGPAPN: 1,"cmnet" t
+QGPAPN: 2,"ims" c
l
+QGPAPN: 3,"" a
e
+QGPAPN: 4,"cmwap","10.6.12.164"
i
+QGPAPN: 5,"" u t
n
Q
OK
e
d
3.5.7. AT+QWDSCFG Wireless Device Service
i
f
AT+QWDSCFG Wireless Device Service
n
Test Command Response
o
AT+QWDSCFG=? +QWDSCFG: "lte_attach_pdn"
+QWDSCFG: "operator_reserved_pco",(range of supported
C
<profileID>s),(range of supported <APN_class>s),(range of
supported <MCC>s),(range of supported <MNC>s),(range of
supported <PCO_ID>s),(range of supported <PCO_ID>s),(range of
supported <PCO_ID>s),(range of supported <PCO_ID>s),(range of
supported <PCO_ID>s),(range of supported <PCO_ID>s),(range of
supported <PCO_ID>s),(range of supported <PCO_ID>s),(range of
supported <PCO_ID>s),(range of supported <PCO_ID>s),(range of
supported <PCO_ID>s)
…
OK
Maximum Response Time 300 ms
Characteristics /
RG50xQ&RM5xxQ_Series_Network_Application_Note 67 / 136

5G Module Series
3.5.7.1. AT+QWDSCFG="lte_attach_pdn" Set LTE Attachment PDN
This command gets and sets LTE attachment PDN.
AT+QWDSCFG="lte_attach_pdn" Set LTE Attachment PDN
Read Command Response
AT+QWDSCFG="lte_attach_pdn"[,<c If the optional parameters are omitted, query the current
ount>,<PDN>] setting.
+QWDSCFG: "lte_attach_pdn",<count>,<PDN>
OK
l
If the optional parameters are specified, set LTE attachment
e
PDN:
OK t
Or c
l
ERROR a
e
Maximum Response Time 300 ms
i
u t
Characteristics / n
Q
Reference
e
3GPP TS 27.007
d
Parameter
i
f
n
<count> Integer type. Specifies a particular PDP context definition
<PDN> String type. Specifies PDN profile IDs. Separate with “:”(see AT+CGDCONT).
o
C
Example
AT+QWDSCFG="lte_attach_pdn"
+QWDSCFG: "lte_attach_pdn",1,1
OK
AT+QWDSCFG="lte_attach_pdn",3,2:3:4
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 68 / 136

5G Module Series
3.5.7.2. AT+QWDSCFG="operator_reserved_pco" Set Operator Reserved PCO
This command gets and sets operator reserved PCO.
AT+QWDSCFG="operator_reserved_pco" Set Operator Reserved PCO
Write Command Response
AT+QWDSCFG="operator_reserved_ If the optional parameters are omitted, query the current
pco",<profileID>[,<APN_class>,<MC setting:
C>,<MNC>[,<PCO_ID>[,<PCO_ID>[,< +QWDSCFG: "operator_reserved_pco",<profileID>,<APN
PCO_ID>…]]]] _class>,<MCC>,<MNC>[,<PCO_ID>[,<PCO_ID>[,<PCO_I
D>…]]]
l
OK
e
If the optiontal parameters are specified, query the operator
resecrved PCO:
l
OK a
e
Or
i
uERROR t
n
Maximum Response Time 300 ms
Q
Characteristics / e
Reference
d
3GPP TS 24.008
i
f
n
Parameter
o
<profileID> Integer type. Specifies PDN profile IDs. Range: 1–42.
<APN_cClass> Integer type. Specifies APN class to operator. Range: 0–16.
<MCC> Integer type. Specifies PCO MCC. Range: 0–999.
<MNC> Integer type. Specifies PCO MNC. Range: 0–999.
<PCO_ID> Integer type. Specifies PCO ID in operator reserved PCO. Max count is 11.
Range: 65280–65535.
NOTE
1. If “-“ is returned, it means this parameter is invalid.
2. Configuration will transparently send to Operator Network.
Example
AT+QWDSCFG="operator_reserved_pco",1,13,460,00,65280,65281,65282,65283,65284,65285,6528
6,65287,65288,65289,65290
RG50xQ&RM5xxQ_Series_Network_Application_Note 69 / 136

5G Module Series
OK
AT+QWDSCFG="operator_reserved_pco",1
+QWDSCFG: "operator_reserved_pco",1,13,460,00,65280,65281,65282,65283,65284,65285,65286,
65287,65288,65289,65290
OK
3.6. AT+QNWLOCK Network Cell Lock
AT+QNWLOCK Network Cell Lock l
e
Test Command Response
AT+QNWLOCK=? +QNWLOCK: "tcommon/4g",(range of supported
c
<num_of_cellls>s),<freq>,<pci> l
a
+QNWLOCK: "common/5g",<pci>,<freq>,<scs>,<band>
e
+QNWLOCK: "save_ctrl",(list of supported <lte_ctrl>s),(list of
i
usupported <nr5g_ctrl>s) t
+QNWLOCK: "common/4g_enxt",(range of supported
Q
<num_of_cellls>s),<cell_list>
e
OK d
Maximum Response Time 300 ms
i
f
Characteristics /
n
o
3.6.1. AT+QNWLOCK="common/4g" Lock Module to the Specified 4G Cell
C
AT+QNWLOCK="common/4g" Lock Module to the Specified 4G Cell
Write Command Response
AT+QNWLOCK="common/4g"[ If the optional parameters are omitted, query the current
,<num_of_cells>[,<freq>,<pci>
configuration:
[,...]]]
+QNWLOCK: "common/4g",<num_of_cells>,<freq>,<pci>
OK
If the optional parameters are specified, lock the module to the
specified 4G cell:
OK
If there is any error related to MT functionality:
+CME ERROR: <err>
RG50xQ&RM5xxQ_Series_Network_Application_Note 70 / 136

5G Module Series
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics The saving mechanism is determined by
AT+QNWLOCK="save_ctrl".
Parameter
<num_of_cells> Integer type Number of cells to be locked. Range: 0–10. 0 indicates disabling
locking module to the specified cell.
<freq> Integer type. Cell frequency.
<pci> Integer type. Cell physical ID.
<err> Error codes. See Chapter 1 for details
l
e
NOTE t
c
1. Before executing the command, lock RAT as LTE. l
a
2. When locking the cell, please eensure that the band corresponding to the locked cell is supported by
the module, otherwise the setting cannot take effect.
i
3. This Write Command u can only be executed when the module is in full functtionality (AT+CFUN=1).
n
4. This command is not recommended for commercial use.
Q
e
Example
d
AT+QNWLOCK="common/4g",1,1300,123
i
OK f
n
AT+QNWLOCK="common/4g"
+QNWLOCK: "common/4g",1,1300,123
o
OK C
3.6.2. AT+QNWLOCK="common/5g" Lock Module to the Specified 5G Cell
The lock 5G cell command has strict parameter checks.
⚫ If the range of <pci> exceeds 1008, the UE reports +CME ERROR: invalid nr pci.
⚫ If the specified <scs> is not supported by the module, the UE reports +CME ERROR: invalid nr scs.
⚫ If the specified <band> is not supported by the module, the UE reports +CME ERROR: band not
support.
⚫ If the specified <freq> is not within the range of <band>, the UE reports +CME ERROR: freq not in
band.
⚫ If the UE is not in full functionality, the UE reports +CME ERROR: invalid nwlock initial state.
RG50xQ&RM5xxQ_Series_Network_Application_Note 71 / 136

5G Module Series
AT+QNWLOCK="common/5g" Lock Module to the Specified 5G Cell
Write Command Response
AT+QNWLOCK="common/5g" If the optional parameters are omitted, query the current
[,<pci>,<freq>,<scs>,<band>]
configuration:
+QNWLOCK: "common/5g",<pci>,<freq>,<scs>,<band>
OK
If the optional parameters are specified, lock the module to a
specified 5G cell:
OK
l
If there is any error relaeted to MT functionality:
+CME ERROR: <err>
t
Maximum Response Time 300 ms c
l
The command takes effect immediately. a
e
Characteristics The saving mechanism is determined by
i
uAT+QNWLOCK="save_ctrl". t
n
Q
Parameter
e
<pci> String type. Cell physical ID. 0 indicates disabling locking module to the specified cell.
d
<freq> Integer type. Cell frequency.
<scs> Integer type. NR sub carrier space. Unit: kHz. For FR1 FDD band, please set <scs> to
i
15; for FR1 TDD bandf, please set <scs> to 30; and for FR2 band, please set <scs> to
60 or 120. Ontherwise, an error code may be returned.
15
o
30
C 60
120
240
<band> Integer type. NR5G frequency band.
<err> Error codes. See Chapter 1 for details
NOTE
1. When locking a cell, please make sure that the module supports the frequency band corresponding
to the locked cell, otherwise an error code will be returned.
2. AT+QNWLOCK="common/5g" does not support locking 5G cells of NSA.
3. This Write Command can only be executed when the module is in full functionality (AT+CFUN=1).
4. This command cannot be used together with AT+QNWCFG="nr5g_earfcn_lock".
5. This command is not recommended for commercial use.
RG50xQ&RM5xxQ_Series_Network_Application_Note 72 / 136

5G Module Series
Example
AT+QNWLOCK="common/5g",901,504990,30,41
OK
AT+QNWLOCK="common/5g"
+QNWLOCK: "common/5g",901,504990,30,41
OK
3.6.3. AT+QNWLOCK="save_ctrl" Configure Whether to Save the Locked Cell
AT+QNWLOCK="save_ctrl" Configure Whether to Save the Locked Cell
l
e
Write Command Response
AT+QNWLOCK="save_ctrl"[,< If the optional parameters are omitted, query the current setting:
t
lte_ctrl>,<nr5g_ctrl>]
+QNWLOcCK: "save_ctrl",<lte_ctrl>,<nr5g_ctrl>
l
a
e
OK
i
u t
If the optional parameters are spnecified, configure whether to save
Q
the locked cell:
e
OK
d
If there is any error:
ERROR i
f
Maximum Response Time 300 ms
n
The command takes effect immediately.
Characteristics o
The configuration is not saved.
C
Parameter
<lte_ctrl> Integer type. Whether to save the locked LTE cell.
0 Not save
1 Save
<nr5g_ctrl> Integer type. Whether to save the locked NR5G cell.
0 Not save
1 Save
NOTE
This command is not recommended for commercial use.
RG50xQ&RM5xxQ_Series_Network_Application_Note 73 / 136

5G Module Series
Example
AT+QNWLOCK="save_ctrl",1,1
OK
AT+QNWLOCK="save_ctrl"
+QNWLOCK: "save_ctrl",1,1
OK
3.6.4. AT+QNWLOCK="common/4g_ext" Lock Module to the Specified 4G Cell
AT+QNWLOCK="common/4g_ext" Lock Module to the
l
Specified 4G Cell
e
Write Command Response
AT+QNWLOCK="common/4g_ If the optional parameters are omitted, query the current setting:
t
ext"[,<num_of_cells>[,<cell_lis
+QNWLOCcK: "common/4g_ext",<num_of_cells>,<cell_list>
t>]] l
a
e
OK
i
u t
If the optional parameters are specified, lock the module to the
n
specified 4G cell:
Q
OK
e
If there is any derror related to MT functionality:
+CME ERROR: <err>
i
Maximum Response Time 300 mfs
n
The command takes effect immediately.
Characteristics The saving mechanism is determined by
o
AT+QNWLOCK="save_ctrl".
C
Parameter
<num_of_cells> Integer type. Number of cells to be locked. Range: 0–20. 0 indicates disabling
locking module to the specified cell.
<cell_list> String type without double quotes. Use the colon as a separator to list the cells
to be configured. The parameter format is: freq1:PCI1:…freqx:PCIx.
<err> Error codes. See Chapter 1 for details.
NOTE
1. Before executing the command, lock RAT as LTE.
2. When locking the cell, please ensure that the band corresponding to the locked cell is supported by
the module, otherwise the setting cannot take effect.
3. This Write Command can only be executed when the module is in full functionality (AT+CFUN=1).
RG50xQ&RM5xxQ_Series_Network_Application_Note 74 / 136

5G Module Series
4. This command is not recommended for commercial use.
5. The PCI max support 503.
Example
AT+QNWLOCK="common/4g_ext",2, "1300:123:2525:456"
OK
AT+QNWLOCK="common/4g"
+QNWLOCK: "common/4g",2:1300:123:2525:456
OK
l
e
3.7. AT+QNWCFG Configure and Qutery Network Parameters
c
l
a
e
This command configures and queries network parameters.
i
u t
AT+QNWCFG Configure and Query Network Parameters
n
Test CommaQnd Response
AT+QNWCFG=? … e
+QNWCFG: "lte_cell_id"
d
+QNWCFG: "nr5g_cell_id"
+QNWCFG: "up/down",(range of supported <time_interval>s)
i
+QNWfCFG: "dss_enable",(list of supported <enable>s)
n
+QNWCFG: "lte_dl_tx_mode"
+QNWCFG: "clr_rplmn"
o
+QNWCFG: "dis_rplmnact",(list of supported <mode>s)
C +QNWCFG: "lte_ambr"
+QNWCFG: "nr5g_ambr"
+QNWCFG: "dis_4mimo_enable",(list of supported <enable>s)
+QNWCFG: "encryp_alg_support"
+QNWCFG: "integ_alg_support"
+QNWCFG: "data_roaming",(list of supported <data_roaming>s)
+QNWCFG: "nr5g_earfcn_lock",(range of supported
<EARFCN_count>s),<EARFCN_list>
+QNWCFG: "lte_earfcn_lock",(range of supported
<EARFCN_count>s),<EARFCN_list>
+QNWCFG: "used_algo",(list of supported <enable>s)
+QNWCFG: "nr5g_pref_freq_list",(range of supported
<EARFCN_count>s),<EARFCN_list>
+QNWCFG: "lte_pref_freq_list",(range of supported
<EARFCN_count>s),<EARFCN_list>
RG50xQ&RM5xxQ_Series_Network_Application_Note 75 / 136

5G Module Series
+QNWCFG: "ehplmn_config",(range of supported
<ehplmn_count>s),<ehplmn_list>
+QNWCFG: "rrc_state",(list of supported <enable>s)
+QNWCFG: "lte_mimo_layers"
+QNWCFG: "lte_band_priority",(list of supported <band_list>s)
+QNWCFG: "n5rg_band_priority",(list of supported <band_list>s)
+QNWCFG: "cause7_map_cause14",(list of supported <enable>s)
+QNWCFG: "nr5g_ul_256qam",<enable_fr1>,<enable_fr2>
+QNWCFG: "thin_ui_cfg",(range of supported <enable>s)
+QNWCFG: "lte_pco",(range of supported <URC_cfg>s)
+QNWCFG: "msisdn",<mode>
+QNWCFG: "lte_fgi_fdd",<FGI_FDD >
+QNWCFG: "lte_fgi_tdd",<FGI_lTDD>
e
+QNWCFG: "sysmode"
+QNWCFG: "nitz_tons"
+QNWCFGc: "clr_guti"
l
… a
e
i
uOK
t
Maximum Response Time 300 ms n
Q
Characteristics / e
d
3.7.1. AT+QNWCFG="lte_cell_id" Read Cell ID Under LTE
i
f
This command reads ECGI, ECnI, eNodeB ID under LTE.
AT+QNWCFG="lteo_cell_id" Read Cell ID Under LTE
Write ComCmand Response
AT+QNWCFG="lte_cell_id" +QNWCFG: "lte_cell_id",<ECGI>,<ECI>,<eNodeB_ID>
OK
Maximum Response Time 300 ms
Characteristics /
Parameter
<ECGI> Integer type. E-UTRAN Cell Global Identification (MCC + MNC + ECI).
<ECI> Integer type. E-UTRAN Cell Identification (eNodeB ID + cell ID).
<eNodeB_ID> Integer type. LTE base station ID.
RG50xQ&RM5xxQ_Series_Network_Application_Note 76 / 136

5G Module Series
Example
AT+QNWCFG="lte_cell_id" // Read Cell ID under LTE
+QNWCFG: "lte_cell_id",64F0000D6B5C0,0D6B5C0,0D6B5
OK
AT+QNWCFG="lte_cell_id" // Read Cell ID under non-LTE mode.
OK
3.7.2. AT+QNWCFG="nr5g_cell_id" Read Cell ID Under NR5G SA
This command reads the NCGI, NCI, NR5G base station ID under 5G SA .
l
AT+QNWCFG="nr5g_cell_id" Read Cell ID Undeer NR5G SA
Write Command Response t
AT+QNWCFG="nr5g_cell_id" [+QNWcCFG: "nr5g_cell_id",<NCGI>,<NCI>,<gNodeB_ID>]
l
a
e
OK
i
Maximum Response Time u 300 ms t
n
CharacteristicQs /
e
d
Parameter
i
<NCGI> Integer type. NR Cell fGlobal Identification (MCC + MNC + NCI).
n
<NCI> Integer type. NR Cell Identification (gNodeB ID + cell ID).
<gNodeB_ID> Integer type. NR5G base station ID .
o
C
Example
AT+QNWCFG="nr5g_cell_id" //Read Cell ID under NR5G SA.
+QNWCFG: "nr5g_cell_id",64F000170C23000,170C23000,170C23
OK
AT+QNWCFG="nr5g_cell_id" //Read Cell ID under non-NR5G SA.
OK
3.7.3. AT+QNWCFG="up/down" Get Average Uplink Rate and Downlink Rate in
Delta Time
This command gets average uplink rate and downlink rate in delta time.
RG50xQ&RM5xxQ_Series_Network_Application_Note 77 / 136

5G Module Series
AT+QNWCFG="up/down" Get Average Uplink Rate and Downlink Rate in Delta Time
Write Command Response
AT+QNWCFG="up/down"[,< If the optional parameter is omitted, query the current configuration and
time_interval>] average uplink rate and downlink rate:
+QNWCFG: "up/down",<uplink>,<downlink>,<time_interval>
OK
If the optional parameter is specified, set interval time of automatically
calculating the average rate:
OK
l
e
If there is any error:
ERROR t
c
Maximum Response Time 300 ms l
a
Thee command takes effect immediately.
Characteristics
The configuration is not saved. i
u t
n
Q
Parameter
e
<uplink> Integer type. Average rate of uplink in delta time. Unit: bits/second.
d
<downlink> Integer type. Average rate of downlink in delta time. Unit: bits/second.
<time_interval> Integer type. Time to calculate the average rate automatically. Range:1–60. Default
i
value: 2. Unit: seconfd.
n
Example o
AT+QNWCCFG="up/down" //Query command
+QNWCFG: "up/down",2056,384,2
OK
AT+QNWCFG="up/down",5 //Write command
OK
3.7.4. AT+QNWCFG="dss_enable" Enable/Disable DSS Function
This command enables or disables DSS function.
AT+QNWCFG="dss_enable" Enable/Disable DSS Function
Write Command Response
AT+QNWCFG="dss_enable"[, If the optional parameter is omitted, query the current configuration:
<enable>] +QNWCFG: "dss_enable",<enable>
RG50xQ&RM5xxQ_Series_Network_Application_Note 78 / 136

5G Module Series
OK
If the optional parameter is specified, enable or disable DSS function:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automati cally.
l
e
Parameter
t
c
l
<enable> Integer type. Enable or disable DSS function. a
e
0 Disable
i
1 Enable
u t
n
Q
Example
e
AT+QNWCFG="dss_enable",1 //Enable DSS function.
d
OK
AT+QNWCFG="dss_enable" i //Query whether DSS is enabled
+QNWCFG: "dss_enable",1 f
n
OK
o
C
3.7.5. AT+QNWCFG="lte_dl_tx_mode" Query Downlink Transmission Mode
This command queries downlink transmission mode.
AT+QNWCFG="lte_dl_tx_mode" Query Downlink Transmission Mode
Write Command Response
AT +QNWCFG="lte_dl_tx_mode" +QNWCFG: "lte_dl_tx_mode",<tx_mode>
OK
Maximum Response Time 300 ms
Characteristics /
RG50xQ&RM5xxQ_Series_Network_Application_Note 79 / 136

5G Module Series
Parameter
<tx_mode> Integer type. Downlink transmission mode.
0 Invalid mode
1 Single antenna port 0
2 Transmit diversity
3 Open loop spatial multiplexing
4 Close loop spatial multiplexing
5 Multi-user MIMO
6 Closed loop rank 1 precoding
7 Single antenna port 5
9 Maximum number of modes
l
e
Example
t
AT+QNWCFG="lte_dl_tx_mode" c
l
+QNWCFG: "lte_dl_tx_mode",2 a
e
i
OK u t
n
Q
3.7.6. AT+QNWCFG="clr_rplmn" Clear RPLMNe
This command clears RPLMN. d
AT+QNWCFG="clr_rplmn" Clear RPiLMN
f
Write Command nResponse
AT+QNWCFG="clr_rplmn" OK
o
Or
ERROR
C
Maximum Response Time 300 ms
Characteristics /
Example
AT+QNWCFG="clr_rplmn"
OK
3.7.7. AT+QNWCFG="dis_rplmnact" Enable/Disable RPLMNACT
This command enables or disables RPLMNACT.
RG50xQ&RM5xxQ_Series_Network_Application_Note 80 / 136

5G Module Series
AT+QNWCFG="dis_rplmnact" Enable/Disable RPLMNACT
Write Command Response
AT+QNWCFG="dis_rplmnact"[,<m If the optional parameter is omitted, query the current
ode>] configuration:
+QNWCFG: "dis_rplmnact",<mode>
OK
If the optional parameter is specified, enable or disable
RPLMNACT:
OK
If there is any error: l
e
ERROR
Maximum Response Time 300 ms t
c
l
The command takes effect immediately.
Characteristics a
eThe configuration is saved automatically.
i
u t
Parameter n
Q
e
<mode> Integer type. Enable or disable RPLMNACT.
0 Enable
d
1 Disable
i
f
Example n
AT+QNWCFG="dis_orplmnact"
+QNWCFG: "dis_rplmnact",0
C
OK
AT+QNWCFG="dis_rplmnact",1
OK
3.7.8. AT+QNWCFG="lte_ambr" Query LTE AMBR
This command queries the AMBR value of the activated APN in the LTE network.
AT+QNWCFG="lte_ambr" Query LTE AMBR
Write Command Response
AT+QNWCFG="lte_ambr" +QNWCFG: "lte_ambr",<APN_name>,<AMBR_DL>,<AMBR_UL>
[+QNWCFG: "lte_ambr",<APN_name>,<AMBR_DL>,<AMBR_UL>]
[…]
RG50xQ&RM5xxQ_Series_Network_Application_Note 81 / 136

5G Module Series
OK
Maximum Response Time 300 ms
Characteristics /
Parameter
<APN_name> String type. The name of the activated APN.
<AMBR_DL> Integer type. Downlink aggregate maximum bit rate. Unit: Kbps
<AMBR_UL> Integer type. Uplink aggregate maximum bit rate. Unit: Kbps
l
e
Example
t
AT+QNWCFG="lte_ambr" c
l
+QNWCFG: "lte_ambr","cmnet",532840,104000 a
e
+QNWCFG: "lte_ambr","ims",846640,846640
i
u t
OK
n
Q
e
3.7.9. AT+QNWCFG="nr5g_ambr" Query NR5G AMBR
d
This command queries the AMBR value of the activated DNN in the NR5G network.
i
f
AT+QNWCFG="nr5g_ambr" Query NR5G AMBR
n
Write Command Response
o
AT+QNWCFG="nr5g_ambr" +QNWCFG: "nr5g_ambr",<DNN_name>,<unit_DL>,<sessio
n_DL>,<unit_UL>,<session_UL>
C
[+QNWCFG: "nr5g_ambr",<DNN_name>,<unit_DL>,<sessio
n_DL>,<unit_UL>,<session_UL>]
[…]
OK
Maximum Response Time 300 ms
Characteristics /
Parameter
<DNN_name> String type. The name of the activated DNN.
<uint_DL> Integer type. Downlink aggregate maximum bit rate.
1 1 Kbps
RG50xQ&RM5xxQ_Series_Network_Application_Note 82 / 136

5G Module Series
2 4 Kbps
3 16 Kbps
4 64 Kbps
5 256 Kbps
6 1 Mbps
7 4 Mbps
8 16 Mbps
9 64 Mbps
10 256 Mbps
11 1 Gbps
12 4 Gbps
13 16 Gbps
14 64 Gbps l
e
15 256 Gbps
<uint_UL> Integer type. Uplink aggregate maximtum bit rate.
1 1 Kbps c
l
2 4 Kbps a
e
3 16 Kbps
i
4 u64 Kbps
t
5 256 Kbps n
Q
6 1 Mbps
e
7 4 Mbps
8 16 Mbps
d
9 64 Mbps
10 256 Mbps i
f
11 1 Gbps
n
12 4 Gbps
13 o 16 Gbps
14 64 Gbps
C
15 256 Gbps
<session_DL> Integer type. Session-AMBR for downlink. The actual session-AMBR for downlink is
equal to <session_DL> multiplied by <uint_DL>.
<session_UL> Integer type. Session-AMBR for uplink. The actual session-AMBR for uplink is equal to
<session_DL> multiplied by <uint_UL>.
Example
AT+QNWCFG="nr5g_ambr"
+QNWCFG: "nr5g_ambr","ims",3,52429,3,52429
+QNWCFG: "nr5g_ambr","cmnet",3,32768,2,26215
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 83 / 136

5G Module Series
3.7.10. AT+QNWCFG="dis_4mimo_enable" Control 4*MIMO of LTE Band
This command enables or disables 4*MIMO of every LTE band.
AT+QNWCFG="dis_4mimo_enable" Control 4*MIMO of LTE Band
Write Command Response
AT+QNWCFG="dis_4mimo_en If the optional parameter is omitted, query the current configuration:
able"[,<enable>] +QNWCFG: "dis_4mimo_enable",<enable>
OK
If the optional parameter is specified , enable or disable 4*MIMO of
l
LTE band:
e
OK
t
If there isc any error:
l
ERROR a
e
Maximum Response Time 300 ms
i
u t
The command takes effect after the module is reboot.
Characteristics n
QThe configuration is saved automatically.
e
Parameter
d
<enable> Integer type. Enable or disable 4*MIMO functionality.
i
0 Disable f
n
1 Enable
o
Example
C
AT+QNWCFG="dis_4mimo_enable",1 //Enable 4*MIMO of every band
OK
AT+QNWCFG="dis_4mimo_enable" //4*MIMO functionality is enabled
+QNWCFG: "dis_4mimo_enable",1
OK
3.7.11. AT+QNWCFG="encryp_alg_support" Query Supported Encryption
Algorithms
This command retrieves supported encryption algorithms.
RG50xQ&RM5xxQ_Series_Network_Application_Note 84 / 136

5G Module Series
AT+QNWCFG="encryp_alg_support" Query Supported Encryption Algorithm
Write Command Response
AT+QNWCFG="encryp_alg_support" +QNWCFG: "encryp_alg_support",<LTE_enc_algo>,<NR
5G_enc_algo>
OK
Maximum Response Time 300 ms
Characteristics /
Parameter
l
e
<LTE_enc_algo> Integer type. LTE encryption algorithm which includes 8 bits, with each bit
t
corresponding to one EPS encryption algorithm.
c
l
Bit 0 eps_encrypt_alg_128_eea0
a
Bit 1 epse_encrypt_alg_128_eea1
Bit 2 eps_encrypt_alg_128_eea2 i
u t
Bit 3 eps_encrypt_alg_eea3
n
QBit 4 eps_encrypt_alg_eea4
Bit 5 eps_encrypt_alg_eea5 e
Bit 6 eps_encrypt_alg_eea6
d
Bit 7 eps_encrypt_alg_eea7
<NR5G_enc_algo> Integer type. NR5G encryption algorithm which includes 16 bits, with each bit
i
corresponding tof NR5G encryption algorithm.
Bit 0 nnr5g_encrypt_alg_5gea0
Bit 1 nr5g_encrypt_alg_128_5gea1
o
Bit 2 nr5g_encrypt_alg_128_5gea2
CBit 3 nr5g_encrypt_alg_128_5gea3
Bit 4 nr5g_encrypt_alg_5gea4
Bit 5 nr5g_encrypt_alg_5gea5
Bit 6 nr5g_encrypt_alg_5gea6
Bit 7 nr5g_encrypt_alg_5gea7
Bit 8 nr5g_encrypt_alg_5gea8
Bit 9 nr5g_encrypt_alg_5gea9
Bit 10 nr5g_encrypt_alg_5gea10
Bit 11 nr5g_encrypt_alg_5gea11
Bit 12 nr5g_encrypt_alg_5gea12
Bit 13 nr5g_encrypt_alg_5gea13
Bit 14 nr5g_encrypt_alg_5gea14
Bit 15 nr5g_encrypt_alg_5gea15
RG50xQ&RM5xxQ_Series_Network_Application_Note 85 / 136

5G Module Series
Example
AT+QNWCFG="encryp_alg_support" //Retrieve the supported encryption algorithm.
+QNWCFG: "encryp_alg_support",15,15
OK
3.7.12. AT+QNWCFG="integ_alg_support” Query Supported Integrity Algorithm
This command retrieves supported integrity algorithm.
AT+QNWCFG="integ_alg_support" Query Supported Integrity Algorithm
Write Command Response l
e
AT+QNWCFG="integ_alg_support" +QNWCFG: "integ_alg_support",<LTE_inte_algo>,<NR5G
_inte_algo> t
c
l
OK a
e
Maximum Response Time 300 ms
i
u t
Characteristics /
n
Q
e
Parameter
d
<LTE_inte_algo> Integer type. LTE integrity algorithm which includes 8 bits, with each bit
i
corresponding tof one EPS integrity algorithm.
Bit 0 neps_integrity_alg_128_eea0
Bit 1 eps_integrity_alg_128_eea1
o
Bit 2 eps_integrity_alg_128_eea2
Bit 3 eps_integrity_alg_128_eea3
C
Bit 4 eps_integrity_alg_128_eea4
Bit 5 eps_integrity_alg_128_eea5
Bit 6 eps_integrity_alg_128_eea6
Bit 7 eps_integrity_alg_128_eea7
<NR5G_inte_algo> Integer type. NR5G integrity algorithm which includes 16 bits, with each bit
corresponding to NR5G integrity algorithm.
Bit 0 nr5g_integrity_alg_5gea0
Bit 1 nr5g_integrity_alg_128_5gea1
Bit 2 nr5g_integrity_alg_128_5gea2
Bit 3 nr5g_integrity_alg_128_5gea3
Bit 4 nr5g_integrity_alg_5gea4
Bit 5 nr5g_integrity_alg_5gea5
Bit 6 nr5g_integrity_alg_5gea6
Bit 7 nr5g_integrity_alg_5gea7
RG50xQ&RM5xxQ_Series_Network_Application_Note 86 / 136

5G Module Series
Bit 8 nr5g_integrity_alg_5gea8
Bit 9 nr5g_integrity_alg_5gea9
Bit 10 nr5g_integrity_alg_5gea10
Bit 11 nr5g_integrity_alg_5gea11
Bit 12 nr5g_integrity_alg_5gea12
Bit 13 nr5g_integrity_alg_5gea13
Bit 14 nr5g_integrity_alg_5gea14
Bit 15 nr5g_integrity_alg_5gea15
Example
AT+QNWCFG="integ_alg_support" //Retrieve the supported inte grity algorithm.
l
+QNWCFG: "integ_alg_support",14,14
e
OK t
c
l
a
e
3.7.13. AT+QNWCFG="data_roaming " Control Data Roaming
i
u t
This command controls the data roaming preference.
n
Q
AT+QNWCFG="data_roaming" Control Data Roaming
e
Write Command Response
d
AT+QNWCFG="data_roaming"[ If the optional parameter is omitted, query the current configuration:
,<data_roaming>] +QNWCFG: "data_roaming",<data_roaming>
i
f
nOK
o
If the optional parameter is specified, enable or disable data
roaming:
C
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<data_roaming> Integer Type. Preference of data roaming.
0 Data roaming is on
1 Data roaming for international is off
RG50xQ&RM5xxQ_Series_Network_Application_Note 87 / 136

5G Module Series
Example
AT+QNWCFG="data_roaming"
+QNWCFG: "data_roaming",1
OK
AT+QNWCFG="data_roaming",0
OK
3.7.14. AT+QNWCFG="nr5g_earfcn_lock" Lock the NR5G EARFCN
AT+QNWCFG="nr5g_earfcn_lock" Lock the NR5G EARFCN
l
e
Write Command Response
AT+QNWCFG="nr5g_earfcn_lock"[, If the optiontal parameters are omitted, query the current
<EARFCN_count>[,<EARFCN_list>] conficguration:
l
] +QNWCFG: "nr5g_earfcn_lock",<EARFCNa_count>[,<EAR
e
FCN_list>]
i
u t
OK n
Q
e
If the optional parameters are specified, lock the NR5G
EARFCN:
d
OK
i
f
If there is any error:
n
ERROR
Maximum Response Toime 300 ms
The command takes effect immediately.
C
Characteristics The saving mechanism is determined by
AT+QNWLOCK="save_ctrl".
Parameter
<EARFCN_count> Integer type. The number of NR5G EARFCN to be locked. Range: 0–32. If this
parameter is set to 0, <EARFCN_list> is omitted.
<EARFCN_list> String type. Use a colon as a separator to list the NR5G EARFCN to be
locked. Format:
<EARFCN1>:<scs1>:<EARFCN2>:<scs2>:…:<EARFCNx>:<scsx>.
The maximum value of x is 32.
<EARFCNx> Integer type. 5G ARFCN that need to be locked.
<scsx> Integer type. NR sub carrier space. Unit: kHz.
15
RG50xQ&RM5xxQ_Series_Network_Application_Note 88 / 136

5G Module Series
30
60
120
240
For FR1 FDD band, please set <scsx> to 15; for FR1 TDD band, please set
<scsx> to 30; and for FR2 band, please set <scsx> to 60 or 120.
NOTE
This command cannot be used together with AT+QNWLOCK="common/5g".
l
e
Example
t
AT+QNWCFG="nr5g_earfcn_lock"
c
+QNWCFG: "nr5g_earfcn_lock",0 l
a
e
OK
i
AT+QNWCFG="nr5g_ear u fcn_lock",2,630000:15:630000:30 t
n
OK
Q
e
3.7.15. AT+QNWCFG="lte_earfcn_lock" Lock the LTE EARFCN
d
i
AT+QNWCFG="lte_earfcn_lock" f Lock the LTE EARFCN
n
Write Command Response
AT+QNWCFG="lte_earfcn_lock"[,<E If the optional parameters are omitted, query the current
o
ARFCN_count>[,<EARFCN_list>]] configuration:
C +QNWCFG: "lte_earfcn_lock",<EARFCN_count>[,<EARF
CN_list>]
OK
If the optional parameters are specified, lock the LTE
EARFCN:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configurations are saved automatically.
RG50xQ&RM5xxQ_Series_Network_Application_Note 89 / 136

5G Module Series
Parameter
<EARFCN_count> Integer type. The number of LTE EARFCN to be locked. Range: 0–2. If this
parameter is 0, <EARFCN_list> is omitted.
<EARFCN_list> String type. Use a colon as a separator to list the LTE EARFCN to be locked.
If <EARFCN_count> is 2. The parameter format is <EARFCN1>:<EARFCN2>.
If <EARFCN_count> is 1. The parameter format is <EARFCN>.
<EARFCNx> Integer type. LTE ARFCN that need to be locked
NOTE
The EARFCN lock feature is disabled while the modem locks the EARFCN of unsupported band. If
modem attach network by locked EARFCN, it will not transfer to forlmal frequency. The LTE EARFCN
e
lock may cause crash dump, and the Qualcomm won’t fix this issue.
t
c
l
Example
a
e
AT+QNWCFG="lte_earfcn_lock"
i
+QNWCFG: "lte_earfcn_luock",0 t
n
Q
OK
e
AT+QNWCFG="lte_earfcn_lock",2,1350:3590
OK
d
i
3.7.16. AT+QNWCFG="used_algof" Enable/Disable Encryption and Integrity Algorithm
n
This command enables and queries the encryption and integrity algorithms.
o
AT+QNWCFG="used_algo" Enable Encryption and Integrity Algorithms
C
Write Command Response
AT+QNWCFG="used_algo"[,<enable>] If the optional parameter is omitted, query the current
configuration:
+QNWCFG: "used_algo",<enable>[,<rat>,<encryp_alg
o>,<integ_algo>]
OK
If the optional parameter is specified, enable or disable the
encryption and integrity algorithms:
OK
If there is any error:
ERROR
RG50xQ&RM5xxQ_Series_Network_Application_Note 90 / 136

                                                                5G Module Series

Maximum Response Time  300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<enable>  Integer type. Enable or disable features
0  Disable
1  Enable
<rat>  Integer type. Radio access technology.
| -1    NONE  |     |     |     |
| ----------- | --- | --- | --- |
l
| 5    WCDMA  |     |     |     |
| ----------- | --- | --- | --- |
e
| 9    LTE  |     |     |     |
| --------- | --- | --- | --- |
t
| 12    NR5G  |     |     |     |
| ----------- | --- | --- | --- |
c
<encryp_algo>  Integer type. Encryption algorithms of LTE/NR5G  l
a
|       | LTE     |     NR5G  |     |
| ----- | ------- | --------- | --- |
e
| 0      | eea0    |     nea0  |     |
| ------ | ------- | --------- | --- |
i
| 1    u  | eea1    |     nea1  | t   |
| ------- | ------- | --------- | --- |
| 2       | eea2    |     nea2  | n   |
Q
| 3      | eea3_v1130   |   nea3  |     |
| ------ | ------------ | ------- | --- |
e
| 4      | spare4_7   |     spare4_7   |     |
| ------ | ---------- | -------------- | --- |
| 5      | spare3_11  |   d  spare3_7  |     |
| 6      | spare2_16  |     spare2_8   |     |
i
| 7      | spare1_31  |     spare1_11  |     |
| ------ | ---------- | -------------- | --- |
f
<integ_algo>  Integer type. Integrity algorithms of LTE/NR5G.
n
|       | LTE     |     NR5G  |     |
| ----- | ------- | --------- | --- |
0 o
|        | eia0_v920  |     nia0  |     |
| ------ | ---------- | --------- | --- |
| 1      | eia1       |     nia1  |     |
C
| 2      | eia2        |     nia2       |     |
| ------ | ----------- | -------------- | --- |
| 3      | eia3_v1130  |     nia3       |     |
| 4      | spare4_21   |     spare4_6   |     |
| 5      | spare3_29   |     spare3_6   |     |
| 6      | spare2_41   |     spare2_7   |     |
| 7      | spare1_77   |     spare1_10  |     |
Example
AT+QNWCFG="used_algo"
+QNWCFG: "used_algo",0

OK
AT+QNWCFG="used_algo",1
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note                                91 / 136

5G Module Series
AT+QNWCFG="used_algo"
+QNWCFG: "used_algo",1,9,2,2
OK
3.7.17. AT+QNWCFG="nr5g_pref_freq_list" Configure NR5G Preference Frequency
This command configures the preference frequency list of NR5G.
AT+QNWCFG="nr5g_pref_freq_list" Configure NR5G Preference Frequency
Write Command Response
AT+QNWCFG="nr5g_pref_freq_list"[, If the optional parameter is omitted, query the current
l
<EARFCN_count>[,<EARFCN_list>]] configuration: e
+QNWCFG: "nr5g_pref_freq_list",<EARFCN_count>[,<E
t
ARFCN_list>]
c
l
a
eOK
i
u t
If the optional parameter is specified, configure the preference
n
frequency list of NR5G:
Q
OK
e
If thered is any error:
ERROR
i
Maximum Response Time f300 ms
n
The command takes effect after the module is rebooted.
Characteristics
o The configurations are saved automatically.
C
Parameter
<EARFCN_list> String type. Use the colon as a separator to list the NR5G EARFCN to be
configured. The format is:
<EARFCN1>:<scs1>:<EARFCN2>:<scs2>:…:<EARFCNx>:<scsx>
The maximum value of x is 32.
<EARFCN_count> Integer type. The number of EARFCNs to be configured. Range:0–32.
<EARFCNx> Integer type. 5G ARFCN that need to be locked.
<scsx> Integer type. NR sub carrier space. Unit: kHz.
15
30
60
120
240
RG50xQ&RM5xxQ_Series_Network_Application_Note 92 / 136

5G Module Series
For FR1 FDD band, please set <scsx> to 15; for FR1 TDD band, please set
<scsx> to 30; and for FR2 band, please set <scsx> to 60 or 120.
Example
AT+QNWCFG="nr5g_pref_freq_list"
+QNWCFG: "nr5g_pref_freq_list",0
OK
AT+QNWCFG="nr5g_pref_freq_list",2,630000:15:630000:30
OK
l
e
3.7.18. AT+QNWCFG="lte_pref_freq_list" Configure LTE Preference Frequency
t
This command configures the preference frequency list of LTE.
c
l
a
AT+QNWCFG="lte_pref_freq_elist" Configure LTE Preference Frequency
Write Command Response i
u t
AT+QNWCFG="lte_pref_freq_list" If the optional parameter is omitted, query the current
n
[,<EARFCN_Qcount>[,<EARFCN_lis configuration:
t>]] +QNWCFG: "lte_peref_freq_list",<EARFCN_count>[,<EARF
CN_list>]
d
OK
i
f
nIf the optional parameter is specified, configure LTE preference
frequency list:
o
OK
C
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configurations are saved automatically.
Parameter
<EARFCN_list> String type. Use the colon as a separator to list the LTE EARFCN to be
configured. The parameter format is:
<EARFCN1>:<EARFCN2>:…:<EARFCNx>
The maximum value of x is 32.
<EARFCN_count> Integer type. Number of EARFCNs to be configured. Range: 0–32.
<EARFCNx> Integer type. LTE EARFCN.
RG50xQ&RM5xxQ_Series_Network_Application_Note 93 / 136

5G Module Series
Example
AT+QNWCFG="lte_pref_freq_list"
+QNWCFG: "lte_pref_freq_list",0
OK
AT+QNWCFG="lte_pref_freq_list",2,1350:40160
OK
3.7.19. AT+QNWCFG="ehplmn_config" Configure EHPLMN List
This command configures the list of EHPLMN.
l
AT+QNWCFG="ehplmn_config" Configure EHPeLMN List
Write Command Response
t
AT+QNWCFG="ehplmn_config"[,<eh If cthe optional parameter is omitted, query the current
l
plmn_count>[,<ehplmn_list>]] configuration: a
e
+QNWCFG: "ehplmn_config"[,<ehplmn_count>[,<ehplm
i
n_list>]]
u t
n
Q OK
e
If the optional parameter is specified, configure EHPLMN list:
d
OK
i
fIf there is any error:
n
ERROR
Maximum Response Toime 300 ms
The command takes effect after the module is rebooted.
CharacteCristics
The configurations are saved automatically.
Parameter
<ehplmn_count> Integer type. The number of EARFCNs to be locked. Range: 0–20.
<ehplmn_list> String type. Use the colon as a separator to list the EHPLMN to be configured. The
parameter format is <ehplmn1>:<ehplmn2>:…:<ehplmnx>, and the maximum
value of x is 20.
<ehplmnx> Integer type. EHPLMN.
NOTE
1. There are several sources of EHPLMN:
⚫ EF file from SIM card
RG50xQ&RM5xxQ_Series_Network_Application_Note 94 / 136

5G Module Series
⚫ EFS file from the module
⚫ HPLMN
2. This command configures the EFS file.
3. When the EFS file configures the EHPLMN list, it must include HPLMN, otherwise it will be
considered invalid, causing the module to finally select HPLMN as EHPLMN.
4. After the EFS file is configured with a valid EHPLMN list, the module uses the configuration as the
final EHPLMN.
Example
AT+QNWCFG="ehplmn_config"
+QNWCFG: "ehplmn_config",0
l
e
OK
AT+QNWCFG="ehplmn_config",2,46001:46007 t
OK c
l
a
e
3.7.20. AT+QNWCFG="rrc_state" Query RAT and RRC State i
u t
n
This commanQd queries the current RAT and RRC states and sets whether to open URC.
e
AT+QNWCFG="rrc_state" Query RAT and RRC State
d
Write Command Response
AT+QNWCFG="rrc_state"[,<ena If the optional parameter is omitted, query the current setting:
i
ble>] f
n+QNWCFG: "rrc_state",<enable><RAT>,<RRC_state>
o
OK
C
If the optional parameter is specified, set whether to open URC:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
This command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<enable> Integer type. Enable or disable the URC of RAT and RRC state changes is
automatically reported.
0 Disable
RG50xQ&RM5xxQ_Series_Network_Application_Note 95 / 136

                                                                5G Module Series

|        | 1      Enable              |     |     |     |
| ------ | -------------------------- | --- | --- | --- |
| <RAT>  | String type. Current RAT.  |     |     |     |
|        | "no server"  No server     |     |     |     |
|        | "WCDMA"  WCDMA             |     |     |     |
|        | "LTE"    LTE               |     |     |     |
|        | "NR5G"   NR5G              |     |     |     |
<RRC_state>  Integer type. Current RRC state. See the following table for more details.

Table 3: RAT State Corresponding to <RRC_state>
| <RRC_state>  | WCDMA  | LTE  |     | NR5G  |
| ------------ | ------ | ---- | --- | ----- |

l
| 0   | IDLE      | NULL                  | e   | IDLE       |
| --- | --------- | --------------------- | --- | ---------- |
| 1   | CELL_PCH  | IDLE_CAMPEtD_ANYCELL  |     | CONNECTED  |
c
l
| 2   | URA_PCH  | IDLE_CAMPED_NORMAL  |     | INACTIVE CAMPED  |
| --- | -------- | ------------------- | --- | ---------------- |
a
e
| 3   | CELL_FACH  | CONNECTING  |     | /  i |
| --- | ---------- | ----------- | --- | ---- |
|     | u          |             |     | t    |
| 4   | CELL_DCH   | CONNECTED   | n/  |      |
Q
| 5   | /   | RELEASING  | e   | /   |
| --- | --- | ---------- | --- | --- |
d
Example
i
| AT+QNWCFG="rrc_state"  |     | f   |     |     |
| ---------------------- | --- | --- | --- | --- |
+QNWCFG: "rrc_state",1,"LTnE",2

o
OK
  C
| +QNWCFG: "rrc_state","LTE",2  |     |   //URC report  |     |     |
| ----------------------------- | --- | --------------- | --- | --- |

+QNWCFG: "rrc_state","LTE",4

+QNWCFG: "rrc_state","NR5G",0

+QNWCFG: "rrc_state","NR5G",1

3.7.21.  AT+QNWCFG="lte_mimo_layers"  Query LTE MIMO Layers
This command gets LTE uplink and downlink MIMO layers.

RG50xQ&RM5xxQ_Series_Network_Application_Note                                96 / 136

5G Module Series
AT+QNWCFG="lte_mimo_layers" Query LTE MIMO Layers
Write Command Response
AT+QNWCFG="lte_mimo_layers" +QNWCFG: "lte_mimo_layers",<ulmimo>,<dlmimo>
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
Characteristics /
l
e
Parameter
t
<ulmimo> Integer type. Number of uplink MIMO layers.
c
l
<dlmimo> Integer type. Number of downlink MIMO layers.
a
e
i
Example
u t
n
AT+QNWCFG="lte_mimo_layers"
Q
+QNWCFG: "lte_mimo_layers",0,2
e
OK d
i
f
3.7.22. AT+QNWCFG="lte_band_priority" Set LTE Band Priority
n
This command sets the LTE band priority.
o
AT+QNWCFG="lte_band_priority" Set LTE Band Priority
C
Write Command Response
AT+QNWCFG="lte_band_priority"[,< If the optional parameter is omitted, query the current setting:
band_list>] +QNWCFG: "lte_band_priority",<band_list>
OK
If the optional parameter is specified, set the LTE band
priority:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
RG50xQ&RM5xxQ_Series_Network_Application_Note 97 / 136

5G Module Series
This command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<band_list> String type. The LTE band priority is separated by the separator ":" from high priority
to low priority. Range of the actual configurable band: 0–255. For specific supported
bands, see 3GPP 36.101. If the configured band is invalid, the priority will be skipped.
NOTE
If the LTE band priority is not set before, 0 will be re turned when you execute
AT+QNWCFG="lte_band_priority". l
e
t
Example
c
l
a
AT+QNWCFG="lte_band_priority" //Query the current LTE band priority.
e
+QNWCFG: "lte_band_priority",7:4:40
i
u t
OK n
Q
AT+QNWCFG="lte_band_priority",38:40:41:7:4 //Set LTE band priority to 38,40,41,7,4, where 38
e
is the highest priority and 4 is the lowest.
OK d
i
f
3.7.23. AT+QNWCFG="nr5g_band_priority" Set NR5G Band Priority
n
This command sets the NR5G band priority.
o
AT+QNWCFG="nr5g _band_priority" Set NR5G Band Priority
C
Write Command Response
AT+QNWCFG="nr5g_band_priority"[, If the optional parameter is omitted, query the current setting:
<band_list>] +QNWCFG: "nr5g_band_priority",<band_list>
OK
If the optional parameter is specified, set the 5G band priority:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
This command takes effect immediately.
Characteristics
The configuration is saved automatically.
RG50xQ&RM5xxQ_Series_Network_Application_Note 98 / 136

5G Module Series
Parameter
<band_list> String type without double quotes. The 5G band priority is separated by the separator
":" from high priority to low priority. Range of the actual configurable band: 0–255.
For specific supported bands, see 3GPP 38.101. If the configured band is invalid,
the priority will be skipped.
NOTE
If the 5G band priority is not set before, 0 will be returned when you execute
AT+QNWCFG="nr5g_band_priority".
l
Example e
AT+QNWCFG="nr5g_band_priority" t//Query the current 5G band priority.
+QNWCFG: "nr5g_band_priority",7:4:40 c
l
a
e
OK
i
AT+QNWCFG="nr5g_banud_priority",38:40:41:7:4 //Set 5G band priority to t38,40,41,7,4, where 38
is the highest pnriority and 4 is the lowest.
Q
OK
e
d
3.7.24. AT+QNWCFG="cause7_map_cause14" Enable/Disable to Map cause7 to
i
cause14 f
n
This command enables or disables to map cause7 to cause14.
o
AT+QNWCFG="cause7_map_cause14" Enable/Disable to Map cause7 to cause14
C
Write Command Response
AT+QNWCFG="cause7_map_cause14" If the optional parameter is omitted, query the current
[,<enable>] setting:
+QNWCFG: "cause7_map_cause14",<enable>
OK
If the optional parameter is specified, enable or disable to
map cause7 to cause14:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
RG50xQ&RM5xxQ_Series_Network_Application_Note 99 / 136

5G Module Series
This command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<enable> Integer type. Enable or disable to map cause7 to cause14.
0 Disable
1 Enable
Example
AT+QNWCFG="cause7_map_cause14"
+QNWCFG: "cause7_map_cause14",0
l
e
OK
t
c
l
a
3.7.25. AT+QNWCFG="nr5g_uel_256qam" Enable/Disable NR5G UL 256QAM
i
This command enables or udisables NR5G UL 256QAM for FR1 and FR2. t
n
AT+QNWCQFG="nr5g_ul_256qam" Enable/Disable NR5G UL 256QAM
e
Write Command Response
AT+QNWCFG="nr5g_ul_256qam"[,<ena If the optional parameters are omitted, query the current
d
ble_fr1>[,<enable_fr2>]] setting:
i+QNWCFG: "nr5g_ul_256qam",<enable_fr1>,<enable
f
_fr2>
n
o OK
C
If <enable_fr2> is omitted and <enable_fr1> is specified,
enable or disable NR5G UL 256QAM for FR1:
OK
If the optional parameters are specified, enable or disable
NR5G UL 256QAM for FR1 and FR2:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configurations are saved automatically.
RG50xQ&RM5xxQ_Series_Network_Application_Note 100 / 136

5G Module Series
Parameter
<enable_fr1> Integer type. Enable or disable NR5G UL 256QAM for FR1.
0 Disable
1 Enable
<enable_fr2> Integer type. Enable or disable NR5G UL 256QAM for FR2.
0 Disable
1 Enable
NOTE
The parameters of this command should match the RF capability of the modem. For example, if the
l
modem does not support FR2 and <enable_fr2> is configured, this scenario is invalid for the modem,
e
thus the configurations will not take effect.
t
c
l
Example a
e
AT+QNWCFG="nr5g_ul_256qam" i
u t
+QNWCFG: "nr5g_ul_256qam",1,1
n
Q
OK e
AT+QNWCFG="nr5g_ul_256qam",0
OK d
AT+QNWCFG="nr5g_ul_256qam",0,1
i
OK f
n
3.7.26. AT+QNWCoFG="thin_ui_cfg" Configure Default Operating Mode After Power-
upC
This command sets and gets the default operating mode after the module powers up.
AT+QNWCFG="thin_ui_cfg" Default Power-up Operating Mode
Write Command Response
AT+QNWCFG="thin_ui_cfg",[<enable If the optional parameter is omitted, query the current setting:
>] +QNWCFG="thin_ui_cfg",<enable>
OK
If the optional parameter is specified, set the default power-up
operating mode:
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 101 / 136

5G Module Series
If there is any error:
ERROR
Maximum Response Time 300 ms
This command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<Enable> Integer type. Default power-up operating mode.
0 The default power-up operating mode is low power mode
1 The default power-up operating mode is online mode
l
e
NOTE
t
1. The configuration may change with the change of MBN.
c
2. The operating mode after power-up may be changed by the driver. l
a
e
i
Example u t
n
AT+QNWCFQG="thin_ui_cfg"
+QNWCFG: "thin_ui_cfg",1 e
d
OK
AT+QNWCFG="thin_ui_cfg",0
i
OK f
n
o
3.7.27. AT+QNWCFG="lte_pco" Query LTE PCO Information
C
This command queries LTE PCO (Protocol Configuration Options) information.
AT+QNWCFG="lte_pco" Query LTE PCO information
Write Command Response
AT+QNWCFG="lte_pco",[<URC_cfg>] If the optional parameter is omitted, query the current setting:
[+QNWCFG: "lte_pco",<CID>,<IP_type>,<PCO_ID>,<PC
O_contents>]
[+QNWCFG: "lte_pco",<CID>,<IP_type>,<PCO_ID>,<PC
O_contents>]
[…]
OK
If the optional parameter is specified to 0 or 1, set the URC
configuration:
RG50xQ&RM5xxQ_Series_Network_Application_Note 102 / 136

                                                                5G Module Series

OK

|     |     |     | If  the  optional parameter  | is  specified  | to  2,  get  | the  URC  |
| --- | --- | --- | ---------------------------- | -------------- | ------------ | --------- |
configuration:
+QNWCFG: "lte_pco"[,<URC_cfg>]

OK

If there is any error:
ERROR
| Maximum Response Time  |     |     | 300 ms  |     |     |     |
| ---------------------- | --- | --- | ------- | --- | --- | --- |

This command takes effect immediately.
l
Characteristics
The configuratioen is saved automatically.
t
Parameter
c
l
Integer type. See AT+CGDCONT. If <CID> is 0, it means the coraresponding
| <CID>    |     |     |     |     |     |     |
| -------- | --- | --- | --- | --- | --- | --- |
e
|       |     <CID> cannot be obtained.  |     |     |     |     |     |
| ----- | ------------------------------ | --- | --- | --- | --- | --- |
i
| <IP_type>  |     Strinug type. IP version information.  |                 |     | t   |     |     |
| ---------- | ------------------------------------------ | --------------- | --- | --- | --- | --- |
|            |     "IPV4"                                 |   IP version 4  |     | n   |     |     |
Q
|       |     "IPV6"  |   IP version 6  |     |     |     |     |
| ----- | ----------- | --------------- | --- | --- | --- | --- |
e
|           |     "IPV4V6"                            | IP version 4 and IP version 6  |     |     |     |     |
| --------- | --------------------------------------- | ------------------------------ | --- | --- | --- | --- |
| <PCO_ID>  |     String type. Hex string of PCO ID.  |                                |     |     |     |     |
d
| <PCO_contents>  | String Format. Hex string of PCO contents.  |     |     |     |     |     |
| --------------- | ------------------------------------------- | --- | --- | --- | --- | --- |
Integer type. Enable ori disable LTE PCO URC.
| <URC_cfg>  |     |     |     |     |     |     |
| ---------- | --- | --- | --- | --- | --- | --- |
f
|       |     0  | Disable LTE PCO unsolicited result code  |     |     |     |     |
| ----- | ------ | ---------------------------------------- | --- | --- | --- | --- |
n
|       |     1  | Enable LTE PCO unsolicited result code:  |     |     |     |     |
| ----- | ------ | ---------------------------------------- | --- | --- | --- | --- |
          o  +QNWCFG: "lte_pco",<CID>,<IP_type>,<PCO_ID>,<PCO_contents>
          2  Query current unsolicited result code configurations
C
Example
| AT+QNWCFG="lte_pco"   |     | //Query the current configuration.  |     |     |     |     |
| --------------------- | --- | ----------------------------------- | --- | --- | --- | --- |
+QNWCFG: "lte_pco",1,"IPV4V6",8021,030000108106DA684E0283063AF20202
+QNWCFG: "lte_pco",1,"IPV4V6",000D,DA684E02
+QNWCFG: "lte_pco",1,"IPV4V6",000D,3AF20202
+QNWCFG: "lte_pco",1,"IPV4V6",0003,24088000C00000000000000000008888
+QNWCFG: "lte_pco",1,"IPV4V6",0003,24088000C00400000000000000008888
+QNWCFG: "lte_pco",4,"IPV6",8021,03000004
+QNWCFG: "lte_pco",4,"IPV6",0001,24088141C00100000000000000003006
+QNWCFG: "lte_pco",4,"IPV6",0001,24088141C00100000000000000003031
+QNWCFG: "lte_pco",4,"IPV6",0001,24088141C00100000000000000003000
+QNWCFG: "lte_pco",4,"IPV6",0001,24088141C00100000000000000003012
+QNWCFG: "lte_pco",3,"IPV4V6",8021,030000108106DA684E0283063AF20202
RG50xQ&RM5xxQ_Series_Network_Application_Note                                103 / 136

5G Module Series
+QNWCFG: "lte_pco",3,"IPV4V6",000D,DA684E02
+QNWCFG: "lte_pco",3,"IPV4V6",000D,3AF20202
+QNWCFG: "lte_pco",3,"IPV4V6",0003,24088000C00000000000000000008888
+QNWCFG: "lte_pco",3,"IPV4V6",0003,24088000C00400000000000000008888
OK
AT+QNWCFG="lte_pco",0 //Turn off URC of LTE PCO
OK
AT+QNWCFG="lte_pco",1 //Turn on URC of LTE PC
OK
AT+QNWCFG="lte_pco",2 //Query URC configuration of LTE PCO
+QNWCFG: "lte_pco",1
l
e
OK
t
c
l
3.7.28. AT+QNWCFG="msisdn" Query MSISDN From the Network
a
e
This command configures MSISDN request and queries MSISDN from the networki.
u t
AT+QNWCFG="msisdn" Query MSISDN From the Netwnork
Q
Write Command Response
e
AT+QNWCFG ="msisdn"[,<mode>] If the optional parameter is omitted, query the MSISDN and
MSISdDN Request configuration:
+QNWCFG: "msisdn",<mode>,<MSISDN_num>,<type>
i
f
nOK
o
If the optional parameter is specified, configure MSISDN request:
OK
C
If there is any error:
ERROR
Maximum Response Time 300 ms
This command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
Parameter
<mode> Integer type. Configure the MSISDN request of the first profile.
0 Use the default configuration in MBN.
1 Force to enable the MSISDN request of the first profile
2 Force to disable the MSISDN Request of the first profile
<MSISDN_num> String type. MSISDN from the network
RG50xQ&RM5xxQ_Series_Network_Application_Note 104 / 136

5G Module Series
<type> Integer type. Address type of MSISDN number
129 Unknow
145 International type (contains the character "+")
NOTE
When MSISDN Request is not enabled, this command can be used normally only when the module is
registered with IMS. If the MSISDN Request is enabled and the network supports the request, the
module can also be used normally when it is under LTE and has triggered the LTE network registration
process.
l
Example
e
AT$QCPCOMSISDN? //Confirm whether the first profile is enabled MSISDN Request
t
$QCPCOMSISDN: 1,0,0,0,0,0,0,0,0,0,0,0
c
l
$QCPCOMSISDN: 2,0,0,0,0,0,0,0,0,0,0,0
a
$QCPCOMSISDN: 3,0,0,0,0,0,0,0,0e,0,0,0
$QCPCOMSISDN: 4,0,0,0,0,0,0,0,0,0,0,0 i
u t
$QCPCOMSISDN: 5,0,0,0,0,0,0,0,0,0,0,0
n
Q
OK e
AT+QNWCFG="msisdn" //Query the MSISDN and MSISDN Request configuration
+QNWCFG: "msisdn",0,"+8610000000000",145 d
i
OK f
AT+QNWCFG="msisdn",1 n //Enable the MSISDN Request of the first profile
OK
o
//Restart the module
AT$QCPCOMSISDN? //Confirm whether the first profile is enabled MSISDN Request
C
$QCPCOMSISDN: 1,1,0,0,0,0,0,0,0,0,0,0
$QCPCOMSISDN: 2,0,0,0,0,0,0,0,0,0,0,0
$QCPCOMSISDN: 3,0,0,0,0,0,0,0,0,0,0,0
$QCPCOMSISDN: 4,0,0,0,0,0,0,0,0,0,0,0
$QCPCOMSISDN: 5,0,0,0,0,0,0,0,0,0,0,0
OK
AT+QNWCFG="msisdn" //Query the MSISDN and MSISDN Request configuration
+QNWCFG: "msisdn",1,"+8610000000000",145
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 105 / 136

5G Module Series
3.7.29. AT+QNWCFG="lte_fgi_fdd" Configure LTE FGI for FDD Bands
This command sets the LTE FGI for FDD bands.
AT+QNWCFG="lte_fgi_fdd" Configure LTE FGI for FDD Bands
Write Command Response
AT+QNWCFG="lte_fgi_fdd"[,<FGI_FD If the optional parameter is omitted, query the current setting:
D>] +QNWCFG: "lte_fgi_fdd",<FGI_FDD>
OK
If the optional parameter is spe cified, set the LTE FGI for FDD
l
bands:
e
OK
t
If thcere is any error:
l
ERROR a
e
Maximum Response Time 300 ms
i
u t
This command takes effect after the module is rebooted.
Characteristics n
Q The configuration is saved automatically.
e
Parameter
d
<FGI_FDD> String type. Use the colon as a separator to list the LTE FGI for FDD to be configured.
i
Format: <FGI_value1>:<FGI_value2>:…:<FGI_valuex>.
f
<FGI_valuex> Integer type. The LTE FGI for FDD.
n
o
Example
C
AT+QNWCFG="lte_fgi_fdd",2:5:8:36:110
OK
AT+QNWCFG="lte_fgi_fdd"
+QNWFCFG: "lte_fgi_fdd",2:5:8:36:110
OK
NOTE
1. The default configuration is set to none because the different Operator has different configuration.
2. This configuration must be set according to 3GPP Release16 TS36.331.
3. The FGI 43–64, 117–132 are undefined and the combinations of some FGIs are shown in the table
below. If the target FGI would be set to 1, the related FGI must be set to 1 at the same time.
RG50xQ&RM5xxQ_Series_Network_Application_Note 106 / 136

                                                                5G Module Series

Table 4: Combinations of Some FGIs
| FGI  |     | Related FGI  |     |
| ---- | --- | ------------ | --- |
| 7    |     | 3            |     |
| 5    |     | 4            |     |
| 22   |     | 5            |     |
| 23   |     | 9            |     |
| 24   |     | 11           |     |

| 26  |     | 12  | l   |
| --- | --- | --- | --- |
e
| 25  |     | 13  |     |
| --- | --- | --- | --- |
t
| 22/23/24/26/39  |     | c15  |     |
| --------------- | --- | ---- | --- |
l
a
| 41  | e   | 15  |     |
| --- | --- | --- | --- |
i
u t
| 5   |     | 17  |     |
| --- | --- | --- | --- |
n
Q
| 5&25  |     | 18  |     |
| ----- | --- | --- | --- |
e
| 5&22/23/24/26/33/34/35/36/37  |     | 19  |     |
| ----------------------------- | --- | --- | --- |
d
| 8   |     | 27  |     |
| --- | --- | --- | --- |
i
f
| 13  |     | 30  |     |
| --- | --- | --- | --- |
n
| 5&22  |     | 33  |     |
| ----- | --- | --- | --- |
o
| 5&23  |     | 34  |     |
| ----- | --- | --- | --- |
C
| 5&24     |     | 35   |     |
| -------- | --- | ---- | --- |
| 5&26     |     | 36   |     |
| 5&22/39  |     | 37   |     |
| 39       |     | 38   |     |
| 38       |     | 40   |     |
| 2&103    |     | 105  |     |
| 1&103    |     | 107  |     |
| 22       |     | 114  |     |

RG50xQ&RM5xxQ_Series_Network_Application_Note                                107 / 136

5G Module Series
3.7.30. AT+QNWCFG="lte_fgi_tdd" Configure LTE FGI for TDD Bands
This command sets the LTE FGI for TDD bands.
AT+QNWCFG="lte_fgi_fdd" Configure LTE FGI for TDD Bands
Write Command Response
AT+QNWCFG="lte_fgi_tdd"[,<FGI_TD If the optional parameter is omitted, query the current setting:
D>] +QNWCFG: "lte_fgi_tdd",<FGI_TDD>
OK
If the optional parameter is spe cified, set the LTE FGI for TDD
l
bands:
e
OK
t
If thcere is any error:
l
ERROR a
e
Maximum Response Time 300 ms
i
u t
This command takes effect after the module is rebooted.
Characteristics n
Q The configuration is saved automatically.
e
Parameter
d
<FGI_TDD> String type. Use the colon as a separator to list the LTE FGI for TDD to be configured.
i
Format: <FGI_value1>:<FGI_value2>:…:<FGI_valuex>.
f
<FGI_valuex> Integer type. The LTE FGI for TDD.
n
o
NOTE
C
1. The default configuration is set to none because the different Operator has different configuration.
2. This configuration must be set according to 3GPP Release16 TS36.331.
3. The FGI 43–64, 117–132 are undefined and the combinations of some FGIs are shown in Table 4
above. If the target FGI would be set to 1, the related FGI must be set to 1 at the same time.
Example
AT+QNWCFG="lte_fgi_tdd",2:5:8:36:110
OK
AT+QNWCFG="lte_fgi_tdd"
+QNWFCFG: "lte_fgi_tdd",2:5:8:36:110
OK
RG50xQ&RM5xxQ_Series_Network_Application_Note 108 / 136

                                                                5G Module Series

3.7.31.  AT+QNWCFG="sysmode"  Query System Mode and Sub-mode
This command queries system mode and sub-mode.
AT+QNWCFG="sysmode"  Query System Mode and Sub-mode
| Write Command        |     |     |     | Response                      |     |     |
| -------------------- | --- | --- | --- | ----------------------------- | --- | --- |
| AT+QNWCFG="sysmode"  |     |     |     | +QNWCFG: <sysmode>,<submode>  |     |     |

OK
| Maximum Response Time  |     |     |     | 300 ms  |     |     |
| ---------------------- | --- | --- | --- | ------- | --- | --- |
| Characteristics        |     |     |     | /       |     |     |
l
e
| Parameter  |     |     |     | t   |     |     |
| ---------- | --- | --- | --- | --- | --- | --- |
c
l
| <sysmode >   | Integer type. System mode value in decimal  |     |     |     |     | a   |
| ------------ | ------------------------------------------- | --- | --- | --- | --- | --- |
e
|     |     0x0   |        | NULL bearer  |     |     |     |
| --- | --------- | ------ | ------------ | --- | --- | --- |
i
|     |     0x1   |        | 3GPP WCDMA  |     |     |     |
| --- | --------- | ------ | ----------- | --- | --- | --- |
|     |           | u      |             |     | t   |     |
|     |     0x2   |        | 3GPP GERAN  |     |     |     |
n
|     |   Q  0x3   |       | 3GPP LTE       |     |     |     |
| --- | ---------- | ----- | -------------- | --- | --- | --- |
|     |     0x4    |       | 3GPP TDSCDMAe  |     |     |     |
|     |     0x5    |       | 3GPP WLAN      |     |     |     |
d
|     |     0x6   |       | 3GPP 5G                    |     |     |     |
| --- | --------- | ----- | -------------------------- | --- | --- | --- |
|     |     0x7   |       | iWLiAN over 3GPP Cellular  |     |     |     |
f3GPP maximum
|     |     0x64   |       |     |     |     |     |
| --- | ---------- | ----- | --- | --- | --- | --- |
n
|     |     0x65   |       | 3GPP2 1X    |     |     |     |
| --- | ---------- | ----- | ----------- | --- | --- | --- |
|     |     0x66   |       | 3GPP2 HRPD  |     |     |     |
o
|     |     0x67    |       | 3GPP2 EHRPD      |     |     |     |
| --- | ----------- | ----- | ---------------- | --- | --- | --- |
|     | C    0x68   |       | 3GPP2 WLAN       |     |     |     |
|     |     0xC8    |       | 3GPP2 maximum    |     |     |     |
|     |     0xC9    |       | WLAN             |     |     |     |
|     |     0x12C   |       | WLAN maximum */  |     |     |     |
<submode >   Integer type. System sub-mode value in decimal. Bit mask value show in decimal.
        Example: 20890720927744 means 0x100000000000 | 0x10000000000 |
|     |     0x20000000000,    |       |                      |     NR5G | TDD | SUB6  |     |     |
| --- | --------------------- | ----- | -------------------- | ---------------------- | --- | --- |
|     |     0x00              |       | SO Mask Unspecified  |                        |     |     |
|     |     0x01              |       | WCDMA                |                        |     |     |
|     |     0x02              |       | HSDPA                |                        |     |     |
|     |     0x04              |       | HSUPA                |                        |     |     |
|     |     0x08              |       | HSDPAPLUS            |                        |     |     |
|     |     0x10              |       | DC HSDPAPLUS         |                        |     |     |
|     |     0x20              |       | 64 QAM               |                        |     |     |
|     |     0x40              |       | HSPA                 |                        |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                109 / 136

                                                                5G Module Series

|       |   0x80    |       | GPRS  |     |     |
| ----- | --------- | ----- | ----- | --- | --- |
|       |   0x100   |       | EDGE  |     |     |
|       |   0x200   |       | GSM   |     |     |
|       |   0x400   |       | S2B   |     |     |
        0x800         3GPP Limited Service (e.g. WCDMA, LTE, 5G)
|       |   0x1000    |      | 3GPP FDD (e.g. LTE, 5G)    |     |     |
| ----- | ----------- | ---- | -------------------------- | --- | --- |
|       |   0x2000    |      | LTE TDD                    |     |     |
|       |   0x4000    |      | TDSCDMA                    |     |     |
|       |   0x8000    |      | DC HSUPA                   |     |     |
|       |   0x10000   |      | 3GPP CA DL (e.g. LTE, 5G)  |     |     |
|       |   0x20000   |      | 3GPP CA UL (e.g. LTE, 5G)  |     |     |
|       |   0x40000   |      | S2B Limited Service        |     |     |

l
|       |   0x80000   |      | 4.5G  |     |     |
| ----- | ----------- | ---- | ----- | --- | --- |
e
|       |   0x100000      |     | 4.5G+       |     |     |
| ----- | --------------- | --- | ----------- | --- | --- |
|       |   0x0001000000  |     | 1X IS95     | t   |     |
|       |   0x0002000000  |     | 1X IcS2000  |     |     |
l
|       |   0x0004000000  |     | 1X IS2000 REL A  |     | a   |
| ----- | --------------- | --- | ---------------- | --- | --- |
e
|       |   0x0008000000  |     | HDR REV0 DPA  |     |     |
| ----- | --------------- | --- | ------------- | --- | --- |
i
|       |   0x0010u000000  |     | HDR REVA DPA  |     |     |
| ----- | ---------------- | --- | ------------- | --- | --- |
t
|       |   0x0020000000  |     | HDR REVB DPA  |     |     |
| ----- | --------------- | --- | ------------- | --- | --- |
n
Q
|       | 0x0040000000  |     | HDR REVA MPA  |     |     |
| ----- | ------------- | --- | ------------- | --- | --- |
e
|       |   0x0080000000  |     | HDR REVB MPA   |     |     |
| ----- | --------------- | --- | -------------- | --- | --- |
|       |   0x0100000000  |     | HDR REVA EMPA  |     |     |
d
|       |   0x0200000000  |     | HDR REVB EMPA   |     |     |
| ----- | --------------- | --- | --------------- | --- | --- |
|       |   0x0400000000  |     | HDRi REVB MMPA  |     |     |
f
|       |   0x0800000000  |     | HDR EVDO FMC  |     |     |
| ----- | --------------- | --- | ------------- | --- | --- |
n
|       |   0x1000000000   |     | 1X Circuit Switched  |     |     |
| ----- | ---------------- | --- | -------------------- | --- | --- |
|       |   0x10000000000  |     | 5G TDD               |     |     |
o
|       |   0x20000000000  |     | 5G SUB6  |     |     |
| ----- | ---------------- | --- | -------- | --- | --- |
C
|       |   0x40000000000    |     | 5G MMWAVE           |     |     |
| ----- | ------------------ | --- | ------------------- | --- | --- |
|       |   0x80000000000    |     | 5G NSA              |     |     |
|       |   0x100000000000   |     | 5G SA               |     |     |
|       |   0x200000000000   |     | 5G Limited Service  |     |     |
Example
AT+QNWCFG=?
OK
AT+QNWCFG="sysmode"
+QNWCFG: "sysmode",6,2048

OK

RG50xQ&RM5xxQ_Series_Network_Application_Note                                110 / 136

5G Module Series
3.7.32. AT+QNWCFG="nitz_ons" Query PLMN Name from NITZ
This command queries PLMN long name and PLMN short name from NITZ signal.
AT+QNWCFG="nitz_ons" Query PLMN Name From NITZ
Write Command Response
AT+QNWCFG="nitz_ons" +QNWCFG: "nitz_ons",<PLMN_long_name>,<PLMN_short_na
me>
OK
Maximum Response Time 300 ms
Characteristics / l
e
Parameter t
c
l
<PLMN_long_name> String type. The PLMN long name from NITZ.
a
e
<PLMN_short_name> String type. The PLMN short name from NITZ.
i
u t
n
NOTE Q
e
If no NITZ signal from the network is received, the command returns an empty string.
d
i
Example
f
n
AT+QNWCFG="nitz_ons"
+QNWCFG: "nitz_ons","","" //No NITZ information was sent from the network
o
OK C
AT+QNWCFG="nitz_ons"
+QNWCFG: "nitz_ons","Smartfren Network","Smartfren"
OK
3.7.33. AT+QNWCFG="clr_guti" Clear GUTI
This command clears GUTI.
AT+QNWCFG="clr_guti" Clear GUTI
Write Command Response
AT+QNWCFG="clr_guti" OK
Or
ERROR
RG50xQ&RM5xxQ_Series_Network_Application_Note 111 / 136

5G Module Series
Maximum Response Time 300 ms
Characteristics The command takes effect immediately.
NOTE
The command must be sent while the modem is offline. You can set modem offline mode with
AT+CFUN=4.
Example
AT+CFUN=4
l
OK e
AT+QNWCFG="clr_guti"
t
OK
c
l
AT+CFUN=1
a
OK e
i
u t
n
Q
3.8. AT+QNWPREFCFG Configure Network Searching Preferences
e
d
This command configures the network searching preferences.
i
AT+QNWPREFCFG Configure Nfetwork Searching Preferences
n
Test Command Response
AT+QNWPREFCFG=? +QNWPREFCFG: "gw_band",(list of supported <gw_band>s)
o
+QNWPREFCFG: "lte_band",(list of supported <LTE_band>s)
C+QNWPREFCFG: "nsa_nr5g_band",(list of supported <NSA_NR5G_b
and>s)
+QNWPREFCFG: "nr5g_band",(list of supported <SA_NR5G_band>s)
+QNWPREFCFG: "mode_pref",(list of supported <mode_pref>s)
+QNWPREFCFG: "srv_domain",(range of supported <srv_domain>s)
+QNWPREFCFG: "voice_domain",(range of supported <voice_domai
n>s)
+QNWPREFCFG: "roam_pref",(list of supported <roam_pref>s)
+QNWPREFCFG: "ue_usage_setting",(list of supported <setting>s)
+QNWPREFCFG: "policy_band"
+QNWPREFCFG: "ue_capability_band"
+QNWPREFCFG: "rat_acq_order",(list of supported <rat_order>s)
+QNWPREFCFG: "nr5g_disable_mode",(list of supported <disable_
mode>s)
+QNWPREFCFG: "rf_band"
RG50xQ&RM5xxQ_Series_Network_Application_Note 112 / 136

                                                                5G Module Series

+QNWPREFCFG: "restore_band"

OK
| Maximum Response Time  |     | 300 ms  |
| ---------------------- | --- | ------- |
| Characteristics        |     | /       |

3.8.1. AT+QNWPREFCFG="gw_band"  WCDMA Band Configuration
This command specifies the preferred WCDMA bands to be searched by UE.

AT+QNWPREFCFG="gw_band"  WCDMA Band Configuration
l
e
Write Command  Response
AT+QNWPREFCFG="gw_band"[,<gw_ If  the  optional  parameter  is  omitted,  query  the  current
t
band>]  configuration:
c
l
+QNWPREFCFG: "gw_band",<gw_band>
a
e
OK  i
u t

n
Q If the optional parameter is specified, configure the preferred
WCDMA bandse to be searched:
OK
d

If there is any error:
i
fERROR
n
Maximum Response Time  300 ms
o The command takes effect immediately.
Characteristics
The configuration is saved automatically.
C
Parameter
<gw_band>   String type. Use the colon as a separator to list the WCDMA Bands to be configured.
|                |   The parameter format is:                     |                  |
| -------------- | ---------------------------------------------- | ---------------- |
|                |   <WCDMA_band1>:<WCDMA_band2>:…:<WCDMA_bandx>  |                  |
| <WCDMA_bandx>  |  Integer type. WCDMA band.                     |                  |
|                |     1                                          | WCDMA 2100 band  |
|                |     2                                          | WCDMA 1900 band  |
|                |     3                                          | WCDMA 1800 band  |
|                |     4                                          | WCDMA 1700 band  |
|                |     5                                          | WCDMA 850 band   |
|                |     6                                          | WCDMA 800 band   |
|                |     8                                          | WCDMA 900 band   |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                113 / 136

5G Module Series
19 WCDMA Japan 850 band
NOTE
When the module locks to WCDMA, an error is reported if <gw_band> is set to null.
Example
AT+QNWPREFCFG="gw_band" //Query the currently configured WCDMA bands of the UE.
+QNWPREFCFG: "gw_band",1:2:3:4:5:6:7:8:9:19
l
OK
e
AT+QNWPREFCFG="gw_band",1:2 //Set WCDMA B1 and B2.
OK t
c
l
a
e
3.8.2. AT+QNWPREFCFG="lte_band" LTE Band Configuration
i
u t
This command specifies the preferred LTE bands to be searched by UE.
n
Q
AT+QNWPREFCFG="lte_band" LTE Band Configuration
e
Write Command Response
d
AT+QNWPREFCFG="lte_band"[,<LTE If the optional parameter is omitted, query the current
_band>] configuration:
i
f+QNWPREFCFG: "lte_band",<LTE_band>
n
OK
o
C If the optional parameter is specified, configure the preferred
LTE bands to be searched:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<LTE_band> String type without double quotes. Use the colon as a separator to list the LTE bands
to be configured. The parameter format is <band1>:<band2>:…:<bandx>.
RG50xQ&RM5xxQ_Series_Network_Application_Note 114 / 136

5G Module Series
<bandx> Integer type. LTE band. The LTE bands supported by the module are: B1, B2, B3, B4,
B5, B7, B8, B12, B13, B14, B17, B18, B19, B20, B25, B26, B28, B29, B30, B32, B34,
B38, 39, B40, B41, B42, B43, B48, B66 and B71.
NOTE
When the module locks to LTE, an error is reported if <LTE_band> is set to null.
Example
AT+QNWPREFCFG="lte_band" //Query the currently configured LTE bands of the UE.
l
+QNWPREFCFG: "lte_band",1:2:3:4:5:7:8:12:13:14:17:18:19:20:25:26:28:29:30:32:34:38:39:40:41:
e
42:66:71
t
c
OK l
a
AT+QNWPREFCFG="lte_band",1:2 //Set LTE B1 and LTE B2.
e
OK
i
u t
n
3.8.3. AT+QQNWPREFCFG="nsa_nr5g_band" NR5G NSA Band Configuration
e
This command specifies the preferred NR5G NSA bands to be searched by UE.
d
AT+QNWPREFCFG="nsa_nr5g_band" NR5G NSA Band Configuration
i
Write Command fResponse
n
AT+QNWPREFCFG="nsa_nr5g_band" If the optional parameter is omitted, query the current
[,<NSA_NR5G_band>] configuration:
o
+QNWPREFCFG: "nsa_nr5g_band",<NSA_NR5G_ban
C d>
OK
If the optional parameter is specified, configure the preferred
NR5G NSA bands to be searched:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
RG50xQ&RM5xxQ_Series_Network_Application_Note 115 / 136

5G Module Series
Parameter
<NSA_NR5G_band> String type. Use the colon as a separator to list the NSA NR5G bands to be
configured. The parameter format is:
<NSA_band1>:<NSA_band2>:…:<NSA_bandx>
<NSA_bandx> Integer type. The NSA NR5G band. The configurable NR5G NSA bands
supported by the module for this command are: n1, n2, n3, n5, n7, n8, n12,
n20, n25, n28, n38, n40, n41, n48, n66, n71, n77, n78, n79, n257, n258, n260
and n261.
NOTE
l
When the module locks to NSA, an error is reported if <NSeA_NR5G_band> is set to null.
t
c
l
Example
a
e
AT+QNWPREFCFG="nsa_nr5g_band" //Query the currently configured NSA NR5G bands of UE
i
+QNWPREFCFG: "nsa_nur5g_band",1:3:7:20:28:40:41:71:77:78:79 t
n
Q
OK
e
AT+QNWPREFCFG="nsa_nr5g_band",1:2 //Set NSA NR5G n1 and NSA NR5G n2.
OK d
i
f
3.8.4. AT+QNWPREFCFG="nr5g_band" NR5G SA Band Configuration
n
This command specifies the preferred NR5G SA bands to be searched by UE.
o
AT+QNWPREFCFG="nr5g_band" NR5G SA Band Configuration
C
Write Command Response
AT+QNWPREFCFG="nr5g_band"[,<S If the optional parameter is omitted, query the current setting:
A_NR5G_band>] +QNWPREFCFG: "nr5g_band",<SA_NR5G_band>
OK
If the optional parameter is specified, configure the preferred
NR5G SA bands to be searched:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
RG50xQ&RM5xxQ_Series_Network_Application_Note 116 / 136

5G Module Series
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<SA_NR5G_band> String type. Use the colon as a separator to list the NR5G SA bands to be
configured. The parameter format is:
<SA_band1>:<SA_band2>:…:<SA_bandx>.
<SA_bandx> The configurable SA NR5G bands supported by the applicable modules for this
command are: n1, n2, n3, n5, n7, n8, n12, n20, n25, n28, n38, n40, n41, n48, n66,
n71, n77, n78, n79.
l
e
NOTE
t
c
When the module locks to NR5G, an error is reported if <SA_NR5G_band> is set to null. l
a
e
i
Example u t
n
AT+QNWPRQEFCFG= "nr5g_band" //Query the currently configured NR5 bands of the UE.
+QNWPREFCFG: "nr5g_band",1:3:7:20:28:40:41:71:77:e78:79
d
OK
AT+QNWPREFCFG= "nr5g_band",1:2 //Set NR5G SA n1 and NR5G SA n2.
i
OK f
n
o
3.8.5. AT+QNWPREFCFG="mode_pref" Network Search Mode Configuration
C
This command specifies the network search mode.
AT+QNWPREFCFG="mdoe_pref" Network Search Mode Configuration
Write Command Response
AT+QNWPREFCFG="mode_pref"[,<m If the optional parameter is omitted, query the current
ode_pref>] configuration:
+QNWPREFCFG: "mode_pref",<mode_pref>
OK
If the optional parameter is specified, configure the network
search mode:
OK
If there is any error:
ERROR
RG50xQ&RM5xxQ_Series_Network_Application_Note 117 / 136

5G Module Series
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<mode_pref> String type. Use the colon as a separator to list the RATs to be configured. The
parameter format is: RAT1:RAT2:…RATN. The RATs supported by the module are as
follows:
AUTO WCDMA & LTE & NR5G
WCDMA WCDMA only
l
LTE LTE only
e
NR5G NR5G only
t
c
l
Example
a
e
AT+QNWPREFCFG="mode_pref" //Query the current configuration.
i
+QNWPREFCFG: "mode_upref",AUTO t
n
Q
OK
e
AT+QNWPREFCFG="mode_pref",LTE //Set RAT to LTE only.
OK
d
AT+QNWPREFCFG="mode_pref",LTE:NR5G //Set RAT to LTE & NR5G.
OK i
f
n
3.8.6. AT+QNWPREFCFG="srv_domain" Service Domain Configuration
o
This command specifies the registered service domain.
C
AT+QNWPREFCFG="srv_domain" Service Domain Configuration
Write Command Response
AT+QNWPREFCFG="srv_domain"[,<s If the optional parameter is omitted, query the current
rv_domain>] configuration:
+QNWPREFCFG: "srv_domain",<srv_domain>
OK
If the optional parameter is specified, configure the service
domain of UE:
OK
If there is any error:
ERROR
RG50xQ&RM5xxQ_Series_Network_Application_Note 118 / 136

5G Module Series
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<srv_domain> Integer type. Service domain of UE.
0 CS only
1 PS only
2 CS & PS
l
Example e
AT+QNWPREFCFG="srv_domain" //Query thte current configuration.
+QNWPREFCFG: "srv_domain",2 c
l
a
e
OK
i
AT+QNWPREFCFG="srvu_domain",1 //Set PS only. t
OK n
Q
e
3.8.7. AT+QNWPREFCFG="voice_domain" Voice Domain Configuration
d
This command specifies the voice domain of UE.
i
f
n
AT+QNWPREFCFG="voice_domain" Voice Domain Configuration
o
Write Command Response
AT+QNWCPREFCFG="voice_domain"[, If the optional parameter is omitted, query the current
<voice_domain>] configuration:
+QNWPREFCFG: "voice_domain",<voice_domain>
OK
If the optional parameter is specified, configure the voice
domain of UE:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
RG50xQ&RM5xxQ_Series_Network_Application_Note 119 / 136

5G Module Series
Parameter
<voice_domain> Integer type. Voice domain of UE.
0 CS voice only
1 IMS PS voice only
2 CS voice preferred
3 IMS voice preferred
Example
AT+QNWPREFCFG="voice_domain" //Query the current configuration
+QNWPREFCFG: "voice_domain",2
l
e
OK
AT+QNWPREFCFG="voice_domain",3 //Set IMS voice preferred
t
OK c
l
a
e
3.8.8. AT+QNWPREFCFG="roam_pref" Roaming Preference Configiuration
u t
n
This command specifies the roaming preference of UE.
Q
e
AT+QNWPREFCFG="roam_pref" Roaming Preference Configuration
Write Command Respodnse
AT+QNWPREFCFG="roam_pref"[,<roa If the optional parameter is omitted, query the current
i
m_pref>] configuration:
f
n+QNWPREFCFG: "roam_pref",<roam_pref>
o
OK
If the optional parameter is specified, configure the roaming
C
preference of UE:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<roam_pref> Integer type. Roaming preference of UE.
1 Roam only on home network
RG50xQ&RM5xxQ_Series_Network_Application_Note 120 / 136

5G Module Series
3 Roam on affiliate network
255 Roam on any network
Example
AT+QNWPREFCFG="roam_pref" //Query the current configuration
+QNWPREFCFG: "roam_pref",255
OK
AT+QNWPREFCFG= "roam_pref",1 //Roam only on home network
OK
l
3.8.9. AT+QNWPREFCFG="ue_usage_setting" UeE Usage Setting Configuration
t
This command specifies the usage setting of UE.
c
l
a
AT+QNWPREFCFG="ue_usage_setting" UE Usage Setting Configuration
e
Write Command Response i
u t
AT+QNWPREFCFG="ue_usage_settin If the optional parameter is omitted, query the current
n
g"[,<setting>] configuration:
Q
+QNWPREFCFG: "ue_usage_setting",<setting>
e
OK d
i
fIf the optional parameter is specified, configure the usage
nsetting of UE:
OK
o
If there is any error:
C
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
Characteristics
The configuration is saved automatically.
Parameter
<setting > Integer type. Usage setting of UE.
0 Voice centric
1 Data centric
Example
AT+QNWPREFCFG="ue_usage_setting" //Query the current configuration
RG50xQ&RM5xxQ_Series_Network_Application_Note 121 / 136

                                                                5G Module Series

+QNWPREFCFG: "ue_usage_setting",1

OK
AT+QNWPREFCFG="ue_usage_setting",0      //Set voice centric
OK

3.8.10.  AT+QNWPREFCFG="policy_band"  Read Carrier Policy Band
This command reads the band configured in the carrier policy.
AT+QNWPREFCFG="policy_band"  Read Carrier Policy Band

Write Command  Response
l
AT+QNWPREFCFG="policy_band"  +QNWPREFCFG:e "gw_band",<gw_band>
+QNWPREFCFG: "lte_band",<LTE_band>
t
+QNWPREFCFG: "nsa_nr5g_band",<NSA_NR5G_band>
c
l
+QNWPREFCFG: "nr5g_band",<SA_NR5G_band>
a
e
OK  i
u t
Maximum Response Time  300 ms
n
Q
Characteristics  /
e
d
Parameter
i
f
<gw_band>     String type. Use the colon as a separator to list the WCDMA bands to be
n
|                |     configured. The parameter format is:          |     |
| -------------- | ------------------------------------------------- | --- |
|                |     o<WCDMA_band1>:<WCDMA_band2>:…:<WCDMA_bandx>  |     |
| <WCDMA_bandx>  |  Integer type. WCDMA band.                        |     |
C
|       |     1   | WCDMA 2100 band       |
| ----- | ------- | --------------------- |
|       |     2   | WCDMA 1900 band       |
|       |     3   | WCDMA 1800 band       |
|       |     4   | WCDMA 1700 band       |
|       |     5   | WCDMA 850 band        |
|       |     6   | WCDMA 800 band        |
|       |     8   | WCDMA 900 band        |
|       |     19  | WCDMA Japan 850 band  |
<LTE_band>   String type. Use the colon as a separator to list the LTE bands to be configured.
 The parameter format is <band1>:<band2>:…:<bandx>.
<bandx>      Integer type. LTE band. The supported bands are B1, B2, B3, B4, B5, B7, B8, B12,
          B13, B14, B17, B18, B19, B20, B25, B26, B28, B29, B30, B32, B34, B38, 39, B40,
|       |     B41, B42, B43, B48, B66 and B71.  |     |
| ----- | ------------------------------------- | --- |
<NSA_NR5G_band> String type. Use the colon as a separator to list the NR5G NSA bands to be
 configured. The parameter format is:
RG50xQ&RM5xxQ_Series_Network_Application_Note                                122 / 136

5G Module Series
<NSA_band1>:<NSA_band1>:…:<NSA_bandx>
<NSA_bandx> Integer type. NR5G NSA band. The supported bands are n1, n2, n3, n5, n7, n8,
n12, n20, n25, n28, n38, n40, n41, n48, n66, n71, n77, n78, n79, n257, n258, n260
and n261.
<SA_NR5G_band> String type. Use the colon as a separator to list the NR5G SA bands to be
configured. The parameter format is:
<SA_band1>:<SA_bandx>:…:<SA_bandx>
<SA_bandx> Integer type. NR5G SA band. The supported bands are n1, n2, n3, n5, n7, n8, n12,
n20, n25, n28, n38, n40, n41, n48, n66, n71, n77, n78, n79.
Example
AT+QNWPREFCFG="policy_band" l
e
+QNWPREFCFG: "gw_band",1:8
+QNWPREFCFG: "lte_band",1:3:8 t
+QNWPREFCFG: "nsa_nr5g_band",78 c
l
+QNWPREFCFG: "nr5g_band",78 a
e
i
OK u t
n
Q
3.8.11. AT+QNWPREFCFG="ue_capability_bande" Query UE Band Capability
This command queries the band configured in thed UE capability information.
AT+QNWPREFCFG="ue_capability_biand" Query UE Band Capability
f
Write Command nResponse
AT+QNWPREFCFG="ue_capabilit +QNWPREFCFG: "gw_band",<gw_band>
o
y_band" +QNWPREFCFG: "lte_band",<LTE_band>
+QNWPREFCFG: "nsa_nr5g_band",<NSA_NR5G_band>
C
+QNWPREFCFG: "nr5g_band",<SA_NR5G_band>
OK
Maximum Response Time 300 ms
Characteristics /
Parameter
<gw_band> String type. Use the colon as a separator to list the WCDMA bands to be
configured. The parameter format is:
<WCDMA_band1>:<WCDMA_band2>:…:<WCDMA_bandx>
<WCDMA_bandx> Integer type. WCDMA band.
1 WCDMA 2100 band
RG50xQ&RM5xxQ_Series_Network_Application_Note 123 / 136

5G Module Series
2 WCDMA 1900 band
3 WCDMA 1800 band
4 WCDMA 1700 band
5 WCDMA 850 band
6 WCDMA 800 band
8 WCDMA 900 band
19 WCDMA Japan 850 band
<LTE_band> String type. Use the colon as a separator to list the LTE bands to be configured.
The parameter format is <band1>:<band2>:…:<bandx>.
<bandx> Integer type. LTE band. The supported bands are B1, B2, B3, B4, B5, B7, B8, B12,
B13, B14, B17, B18, B19, B20, B25, B26, B28, B29, B30, B32, B34, B38, 39, B40,
B41, B42, B43, B48, B66 and B71.
<NSA_NR5G_band> String type. Use the colon as a separator lto list the NR5G NSA bands to be
e
configured. The parameter format is:
<NSA_band1>:<NSA_band1>:…:<NSA_bandx>
t
<NSA_bandx> Integer type. NR5G NcSA band. The supported bands are n1, n2, n3, n5, n7, n8,
l
n12, n20, n25, n28, n38, n40, n41, n48, n66, n71, n77, n78, n79,a n257, n258, n260
e
and n261.
i
<SA_NR5G_band> Struing type. Use the colon as a separator to list th
t
e NR5G SA bands to be
configured. The parameter format is: n
Q <SA_band1>:<SA_bandx>:…:<SA_bandx>
e
<SA_bandx> Integer type. NR5G SA band. The supported bands are n1, n2, n3, n5, n7, n8, n12,
n20, n25, n28, n38, n40, n41, n48, n66, n71, n77, n78, n79.
d
i
Example
f
n
AT+QNWPREFCFG="ue_capability_band"
+QNWPREFCFG: "gw_band",1:8
o
+QNWPREFCFG: "lte_band",1:3:8
+QNWPRCEFCFG: "nsa_nr5g_band",78
+QNWPREFCFG: "nr5g_band",78
OK
3.8.12. AT+QNWPREFCFG="rat_acq_order" Configure RAT Priority
This command configures the RAT acquisition order.
AT+QNWPREFCFG="rat_acq_order" Configure RAT Priority
Write Command Response
AT+QNWPREFCFG="rat_acq_order"[, If the optional parameter is omitted, query the current
<rat_order>] configuration:
+QNWPREFCFG: "rat_acq_order",<rat_order>
RG50xQ&RM5xxQ_Series_Network_Application_Note 124 / 136

5G Module Series
OK
If the optional parameter is specified, configure the RAT
acquisition order:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect after the module is rebooted.
Characteristics
The configuration is saved automatically.
l
e
Parameter
t
<rat_order> String type. Use the coclon as a separator to specify RAT priority. The forlmat is:
RAT1:RAT2:…RATN. The RATs supported by the module are asa follows:
e
WCDMA WCDMA
i
LTE u LTE t
NR5G NR5G n
Q
e
Example
d
AT+QNWPREFCFG= "rat_acq_order" //Query the current RAT order.
+QNWPREFCFG: "rat_acq_order",NR5G:LTiE:WCDMA
f
n
OK
AT+QNWPREFCFG=o "rat_acq_order",LTE:NR5G:WCDMA //Set RAT order priority.
OK
C
AT+CFUN=1,1 //Reset the module.
OK
AT+QNWPREFCFG= "rat_acq_order" //Query the current RAT order.
+QNWPREFCFG: "rat_acq_order",LTE:NR5G:WCDMA
OK
3.8.13. AT+QNWPREFCFG="nr5g_disable_mode" Disable NR5G
This command disables NR5G.
AT+QNWPREFCFG="nr5g_disable_mode" Disable NR5G
Write Command Response
AT+QNWPREFCFG="nr5g_disable_ If the optional parameter is omitted, query the current
RG50xQ&RM5xxQ_Series_Network_Application_Note 125 / 136

5G Module Series
mode"[,<disable_mode>] configuration:
+QNWPREFCFG: "nr5g_disable_mode",<disable_mode>
OK
If the optional parameter is specified, disable NR5G:
OK
If there is any error:
ERROR
Maximum Response Time 300 ms
The command takes effect immediately.
l
Characteristics
The configuratione is saved automatically.
t
Parameter c
l
a
e
<disable_mode> Integer type. Disable NR5G NA/NSA.
i
0 Nueither is disabled t
1 Disable SA n
Q
2 Disable NSA
e
d
Example
AT+QNWPREFCFG="nr5g_disable_mode"i //Query the current configuration.
f
+QNWPREFCFG: "nr5g_disable_mode",0
n
OK o
AT+QNWPREFCFG="nr5g_disable_mode",1 //Disable NR5G SA.
C
3.8.14. AT+QNWPREFCFG="rf_band " Query RF Bands Supported by Module
This command queries the RF bands supported by the module.
AT+QNWPREFCFG="rf_band" Query RF Bands Supported by Module
Write Command Response
AT+QNWPREFCFG="rf_band" +QNWPREFCFG: "gw_band",<gw_band>
+QNWPREFCFG: "lte_band",<LTE_band>
+QNWPREFCFG: "nr5g_band",<SA_NR5G_band>
+QNWPREFCFG: "nsa_nr5g_band",<NSA_NR5G_band>
OK
Maximum Response Time 300 ms
RG50xQ&RM5xxQ_Series_Network_Application_Note 126 / 136

                                                                5G Module Series

Characteristics  /
Parameter
<gw_band>     String type. Use the colon as a separator to list the WCDMA bands to be
|                |     configured. The parameter format is:         |                  |     |     |     |
| -------------- | ------------------------------------------------ | ---------------- | --- | --- | --- |
|                |     <WCDMA_band1>:<WCDMA_band2>:…:<WCDMA_bandx>  |                  |     |     |     |
| <WCDMA_bandx>  |  Integer type. WCDMA band.                       |                  |     |     |     |
|                |     1                                            | WCDMA 2100 band  |     |     |     |
|                |     2                                            | WCDMA 1900 band  |     |     |     |
|                |     3                                            | WCDMA 1800 band  |     |     |     |
l
|       |     4  | WCDMA 1700 band  |     |     |     |
| ----- | ------ | ---------------- | --- | --- | --- |
e
|       |     5   | WCDMA 850 band   |     |     |     |
| ----- | ------- | ---------------- | --- | --- | --- |
|       |     6   | WCDMA 800 band   | t   |     |     |
|       |     8   | WCDMA 900 bancd  |     |     |     |
l
|       |     19  | WCDMA Japan 850 band  |     |     | a   |
| ----- | ------- | --------------------- | --- | --- | --- |
e
<LTE_band>   String type. Use the colon as a separator to list the LTE bands to be configured.
i
|     |  Thue parameter format is <band1>:<band2>:…:<bandx |     |     | t >.  |     |
| --- | -------------------------------------------------- | --- | --- | ----- | --- |
<bandx>      Integer type. LTE band. The supported bandsn are B1, B2, B3, B4, B5, B7, B8, B12,
Q
          B13, B14, B17, B18, B19, B20, B25, B26, B28, B29, B30, B32, B34, B38, 39, B40,
e
|       |     B41, B42, B43, B48, B66 and B71.  |     |     |     |     |
| ----- | ------------------------------------- | --- | --- | --- | --- |
<NSA_NR5G_band> String type. Use the colon as a separator to list the NR5G NSA bands to be
d
 configured. The parameter format is:
|     |  <NSA_band1>:<NSAi_band1>:…:<NSA_bandx>  |     |     |     |     |
| --- | ---------------------------------------- | --- | --- | --- | --- |
f
<NSA_bandx>   Integer type. NR5G NSA band. The supported bands are n1, n2, n3, n5, n7, n8,
n
n12, n20, n25, n28, n38, n40, n41, n48, n66, n71, n77, n78, n79, n257, n258, n260
oand n261.
<SA_NR5G_band>  String type. Use the colon as a separator to list the NR5G SA bands to be
Cconfigured. The parameter format is:
|     | <SA_band1>:<SA_bandx>:…:<SA_bandx>  |     |     |     |     |
| --- | ----------------------------------- | --- | --- | --- | --- |
<SA_bandx>    Integer type. NR5G SA band. The supported bands are n1, n2, n3, n5, n7, n8, n12,
          n20, n25, n28, n38, n40, n41, n48, n66, n71, n77, n78, n79.
Example
AT+QNWPREFCFG="rf_band"     //Query the RF band supported by the module
+QNWPREFCFG: "gw_band","1:5:8"
+QNWPREFCFG: "lte_band","1:3:5:7:8:20:28:32:38:40:41:42:43"
+QNWPREFCFG: "nr5g_band","1:3:5:7:8:20:28:38:40:41:75:76:77:78"
+QNWPREFCFG: "nsa_nr5g_band","1:3:5:7:8:20:28:38:40:41:75:76:77:78"

OK

RG50xQ&RM5xxQ_Series_Network_Application_Note                                127 / 136

5G Module Series
3.8.15. AT+QNWPREFCFG="restore_band" Restore to Default Bands Supported by
Module
This command restores to default bands supported by the module.
AT+QNWPREFCFG="restore_band" Reset to Default Bands Supported by Module
Write Command Response
AT+QNWPREFCFG="restore_band" OK
If there is any error:
ERROR
l
Maximum Response Time 300 ms
e
The command takes effect after the module is rebooted.
Characteristics
t
The configuration is saved automatically.
c
l
a
e
Example
i
u t
AT+QNWPREFCFG="restore_band " //Restore to default bands supported by the module.
n
Q
OK
e
d
3.9. Network Slice Command i
f
n
3.9.1. AT+C5GNSSAI 5GS NSSAI Setting
o
This command Enables updating the default configuration NSSAI stored at MT.
C
AT+C5GNSSAI 5GS NSSAI Setting
Test Command Response
AT+C5GNSSAI=? +C5GNSSAI: (range of supported <dfl_nssai_len>s),(list of
supported <dfl_config_nssai>s)
Read Command Response
AT+C5GNSSAI? +C5GNSSAI: [<dfl_nssai_len>,<dfl_config_nssai>]
OK
Write Command Response
AT+C5GNSSAI=<dfl_nssai_len>,<dfl OK
_config_nssai>
If there is any error:
ERROR
RG50xQ&RM5xxQ_Series_Network_Application_Note 128 / 136

5G Module Series
If there is any error related to MT functionality:
+CME ERROR: <err>
Maximum Response Time 300 ms
Characteristics /
Reference
3GPP TS 27.007
Parameter
<dfl_nssai_len> Integer type. Indicate the length in octets of the default configured NSSAI to be
stored at the MT. l
<dfl_config_nssai> String type in hexadecimal formaet. Dependent of the form, the string can be
separated by dot(s), semicolon(s) and colon(s). This parameter indicates the list
t
of S-NSSAIs included in the default configured NSSAI to be stored by the MT.
c
l
<dfl_config_nssai> is coded as a list of <S-NSSAI>s separated by colons. Refer
a
e
<S-NSSAI> in subclause 10.1.1. This parameter shall not be subject to
conventional character conversion as per AT+CSCS. i
u t
<err> Error codes. See Chapter 1 for details.
n
Q
e
NOTE
d
If the value is an empty string (""), no default configured NSSAI is stored at the MT.
i
f
n
3.9.2. AT+C5GNSSAIRDP Read 5GS NSSAI Dynamic Parameters
o
This command returns the default configured NSSAI, rejected NSSAI for 3GPP access and rejected NSSAI
C
for non-3GPP access stored at the MT.
AT+C5GNSSAIRDP Read 5GS NSSAI Dynamic Parameters
Test Command Response
AT+C5GNSSAIRDP=? +C5GNSSAIRDP: (range of supported <nssai_type>s),(list
of supported <plmn_id>s)
OK
Write Command Response
AT+C5GNSSAIRDP=<nssai_type>,<p [+C5GNSSAIRDP: [<default_configured_nssai_length>,<
lmn_id> default_configured_nssai>[,<rejected_nssai_3gpp_lengt
h>,<rejected_nssai_3gpp>[,<rejected_nssai_non3gpp_le
ngth>,<rejected_nssai_non3gpp>]]]
[+C5GNSSAIRDP: <plmn_id>[,<configured_nssai_lengt
RG50xQ&RM5xxQ_Series_Network_Application_Note 129 / 136

5G Module Series
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
Characteristics / l
e
Reference
3GPP TS 27.007 t
c
l
a
Parameter e
i
u t
<nssai_type> Integer type. Type of NSSAI to be returned.
n
Q 0 Return stored default configured NSSAI only
1 Return storede default configured NSSAI and rejected
NSSAI(s)
d
2 Return stored default configured NSSAI, rejected NSSAI(s)
and configured NSSAI(s)
i
f3 Return stored default configured NSSAI, rejected
n NSSAI(s), configured NSSAI(s) and allowed NSSAI(s)
<plmn_id> String type. MCC and MNC of the PLMN to which the NSSAI
o
information applies. For the format and the encoding of the MCC
C and MNC, see 3GPP TS 23.003. This parameter shall not be
subject to conventional character conversion as per AT+CSCS.
<default_configured_nssai_length> Integer type. Length in octets of the default configured NSSAI
stored at the MT.
<default_configured_nssai> String type in hexadecimal format. Dependent of the form, the
string can be separated by dot(s), semicolon(s) and colon(s).
This parameter indicates the list of S-NSSAIs included in the
default configured NSSAI stored at the MT for the PLMN. The
<default_configured_nssai> is coded as a list of <S-NSSAI>s
separated by colons. Refer <S-NSSAI> in subclause 10.1.1.
This parameter shall not be subject to conventional character
conversion as per AT+CSCS.
<rejected_nssai_3gpp_length> Integer type. Length in octets of the rejected NSSAI associated
with 3GPP access stored at the MT for the serving PLMN.
<rejected_nssai_3gpp> String type in hexadecimal format. Dependent of the form, the
RG50xQ&RM5xxQ_Series_Network_Application_Note 130 / 136

5G Module Series
string can be separated by dot(s), colon(s) and hash(es). This
parameter indicates the list of rejected S-NSSAIs associated
with 3GPP access stored at the MT for the serving PLMN. The
<rejected_nssai_3gpp> is coded as a list of rejected <S-
NSSAI>s separated by colon. For the format and the encoding
of <S-NSSAI>, see also 3GPP TS 23.003. This parameter shall
not be subject to conventional character conversion as per
AT+CSCS. The rejected S-NSSAI has one of the forms:
sst#cause only slice/service type (SST) and reject cause
are present
sst.sd#cause SST and slice differentiator (SD) and reject
cause are pr esent
where cause is a cause vallue according to 3GPP TS 24.501
e
Table 9.11.3.46.1.
<rejected_nssai_non3gpp_length> Integer typet. Length in octets of the rejected NSSAI associated
withc non-3GPP access stored at the MT for the serving PLMN.
l
<rejected_nssai_non3gpp> String type in hexadecimal format. Dependeant of the form, the
e
string can be separated by dot(s), colon(s) and hash(es). This
i
u parameter indicates the list of rejec t ted S-NSSAIs associated
with non-3GPP access stonred at the MT for the serving PLMN.
Q
The <rejected_nssai_non3gpp> is coded as a list of rejected
e
<S-NSSAI>s separated by colon. For the format and the
encoding of <S-NSSAI>, see also 3GPP TS 23.003. This
d
parameter shall not be subject to conventional character
coniversion as per AT+CSCS. The rejected S-NSSAI has one of
f
the forms:
n
sst#cause only slice/service type (SST) and reject cause
o are present
sst.sd#cause SST and slice differentiator (SD) and reject
C cause are present
where cause is a cause value is according to 3GPP TS 24.501
table 9.11.3.46.1.
<configured_nssai_length> Integer type. Length in octets of the configured NSSAI stored at
the MT for the PLMN identified by <plmn_id>.
<configured_nssai> String type in hexadecimal format. Dependent of the form, the
string can be separated by dot(s), semicolon(s) and colon(s).
This parameter indicates the list of configured S-NSSAIs stored
at the MT for the PLMN identified by <plmn_id>. The
<configured_nssai> is coded as a list of <S-NSSAI>s
separated by colons. Refer <S-NSSAI> in subclause 10.1.1.
This parameter shall not be subject to conventional character
conversion as per AT+CSCS.
<allowed_nssai_3gpp_length> Integer type. Length in octets of the allowed NSSAI
associated with 3GPP access stored at the MT for the PLMN
RG50xQ&RM5xxQ_Series_Network_Application_Note 131 / 136

5G Module Series
identified by <plmn_id>.
<allowed_nssai_3gpp> String type in hexadecimal format. Dependent of the form, the
string can be separated by dot(s), semicolon(s) and colon(s).
This parameter indicates the list of allowed S-NSSAIs
associated with 3GPP access stored at the MT for the PLMN
identified by <plmn_id>. The <allowed_nssai_3gpp> is coded
as a list of <S-NSSAI>s separated by colons. Refer <S-NSSAI>
in subclause 10.1.1. This parameter shall not be subject to
conventional character conversion as per AT+CSCS.
<allowed_nssai_non3gpp_length> Integer type. Length in octets of the allowed NSSAI associated
with non-3GPP access stored at the MT for the PLMN identified
by <plmn_id>.
<allowed_nssai_non3gpp> String type in hexadecimall format. Dependent of the form, the
e
string can be separated by dot(s), semicolon(s) and colon(s).
This paramteter indicates the list of allowed S-NSSAIs
asscociated with non-3GPP access stored at the MT for the
l
PLMN identified by <plman_id>. The
e
<allowed_nssai_non3gpp> is coded as a list of <S-NSSAI>s
i
useparated by colons. Refer <S-NS
t
SAI> in subclause 10.1.1.
This parameter shall not bne subject to conventional character
Q
conversion as per AT+CSCS.
e
d
i
f
n
o
C
RG50xQ&RM5xxQ_Series_Network_Application_Note 132 / 136

                                                                5G Module Series

4
| Summary  |     | of  Error  | Codes  |     |
| -------- | --- | ---------- | ------ | --- |

Table 5: General Codes
| Numeric  | Text  |     |     |     |
| -------- | ----- | --- | --- | --- |

| 0   | Phone failure  |     | l   |     |
| --- | -------------- | --- | --- | --- |
e
| 1   | No connection to phone  |     |     |     |
| --- | ----------------------- | --- | --- | --- |
t
c
| 2   | Phone-adaptor link reserved  |     |     | l   |
| --- | ---------------------------- | --- | --- | --- |
a
e
| 3   | Operation not allowed  |     |     |     |
| --- | ---------------------- | --- | --- | --- |
i
u t
| 4   | Operation not supported  |     |     |     |
| --- | ------------------------ | --- | --- | --- |
n
Q
| 5   | PH-SIM PIN required  |     |     |     |
| --- | -------------------- | --- | --- | --- |
e
| 6   | PH-FSIM PIN required  |     |     |     |
| --- | --------------------- | --- | --- | --- |
d
| 7   | PH-FSIM PUK required i |     |     |     |
| --- | ---------------------- | --- | --- | --- |
f
| 10  | SIM not innserted  |     |     |     |
| --- | ------------------ | --- | --- | --- |
SoIM PIN required
11
C
| 12  | SIM PUK required    |     |     |     |
| --- | ------------------- | --- | --- | --- |
| 13  | SIM failure         |     |     |     |
| 14  | SIM busy            |     |     |     |
| 15  | SIM wrong           |     |     |     |
| 16  | Incorrect password  |     |     |     |
| 17  | SIM PIN2 required   |     |     |     |
| 18  | SIM PUK2 required   |     |     |     |
| 20  | Memory full         |     |     |     |
| 21  | Invalid index       |     |     |     |
RG50xQ&RM5xxQ_Series_Network_Application_Note                                133 / 136

                                                                5G Module Series

| 22  | Not found                          |     |     |
| --- | ---------------------------------- | --- | --- |
| 23  | Memory failure                     |     |     |
| 24  | Text string too long               |     |     |
| 25  | Invalid characters in text string  |     |     |
| 26  | Dial string too long               |     |     |
| 27  | Invalid characters in dial string  |     |     |
| 30  | No network service                 |     |     |

l
| 31  | Network timeout  | e   |     |
| --- | ---------------- | --- | --- |
Network not allowed - emergencyt calls only
32
c
l
| 40  | Network personalization PIN required  |     | a   |
| --- | ------------------------------------- | --- | --- |
e
| 41  | Network personalization PUK required  |     | i   |
| --- | ------------------------------------- | --- | --- |
|     | u                                     | t   |     |
n
| 42  | Network subset personalization PIN required  |     |     |
| --- | -------------------------------------------- | --- | --- |
Q
e
| 43  | Network subset personalization PUK required  |     |     |
| --- | -------------------------------------------- | --- | --- |
d
| 44  | Service provider personalization PIN required  |     |     |
| --- | ---------------------------------------------- | --- | --- |
i
| 45  | Service provider perfsonalization PUK required  |     |     |
| --- | ----------------------------------------------- | --- | --- |
n
| 46  | Corporate personalization PIN required  |     |     |
| --- | --------------------------------------- | --- | --- |
o
| 47  | Corporate personalization PUK required  |     |     |
| --- | --------------------------------------- | --- | --- |
C
| 48  | Hidden key required                         |     |     |
| --- | ------------------------------------------- | --- | --- |
| 49  | EAP method not supported                    |     |     |
| 50  | Incorrect parameters                        |     |     |
| 51  | Command implemented but currently disabled  |     |     |
| 52  | Command aborted by user                     |     |     |
53  Not attached to network due to MT functionality restrictions
54  Modem not allowed - MT restricted to emergency calls only
55  Operation not allowed because of MT functionality restrictions
RG50xQ&RM5xxQ_Series_Network_Application_Note                                134 / 136

                                                                5G Module Series

56  Fixed dial number only allowed - called number is not a fixed dial number
| 57  | Temporarily out of service due to other MT usage  |     |
| --- | ------------------------------------------------- | --- |
| 58  | Language/alphabet not supported                   |     |
| 59  | Unexpected data value                             |     |
| 60  | System failure                                    |     |
| 61  | Data missing                                      |     |
| 62  | Call barred                                       |     |

l
| 63  | Message waiting indication subscriptioen failure  |     |
| --- | ------------------------------------------------- | --- |
t
| 100  | Unknown  |     |
| ---- | -------- | --- |
c
l
| 900  | invalid nr pci  | a   |
| ---- | --------------- | --- |
e
| 901  | invalid nr scs  | i   |
| ---- | --------------- | --- |
|      | u t             |     |
n
| 902  | band not support  |     |
| ---- | ----------------- | --- |
Q
e
| 903  | freq not in band  |     |
| ---- | ----------------- | --- |
d
| 904  | already unlock state  |     |
| ---- | --------------------- | --- |
i
| 905  | invalid nwlock initialf state  |     |
| ---- | ------------------------------ | --- |
n
| 906  | nwlock interface abormal  |     |
| ---- | ------------------------- | --- |
o
| 907  | nwlock para error  |     |
| ---- | ------------------ | --- |
C
| 908  | asyn process error  |     |
| ---- | ------------------- | --- |

RG50xQ&RM5xxQ_Series_Network_Application_Note                                135 / 136

                                                                5G Module Series

5
| Appendix  |     | Terms  | and  | Abbreviations  |
| --------- | --- | ------ | ---- | -------------- |

Table 6: Terms and Abbreviations
| Abbreviation  | Description  |     |     |     |
| ------------- | ------------ | --- | --- | --- |

| 5GCN  | 5G Core Network  |     |     | l   |
| ----- | ---------------- | --- | --- | --- |
e
| 5GS  | 5G System  |     |     |     |
| ---- | ---------- | --- | --- | --- |
t
ATtention; this two-character abbreviation is always used to start a command line to
| AT  |     | c   |     |     |
| --- | --- | --- | --- | --- |
l
be sent from TE to TA
a
e
ASCI  Advanced Speech Call Items, including VGCS, VBS and eMLPP
i
u t
| BCD  | Binary C oded Decimal  |     |     |     |
| ---- | ---------------------- | --- | --- | --- |
n
Q
| FR  | Frequency Range  |     |     |     |
| --- | ---------------- | --- | --- | --- |
e
| IMEI  | International Mobile Station Equipment Identity  |     |     |     |
| ----- | ------------------------------------------------ | --- | --- | --- |
d
ITU-T  International Telecommunication Union Telecommunications Standardization Sector
i
| ME  | Mobile Equipment  | f   |     |     |
| --- | ----------------- | --- | --- | --- |
n
| MT  | Mobile Termination  |     |     |     |
| --- | ------------------- | --- | --- | --- |
o
| NG-RAN  | Next Generation Radio Access Network  |     |     |     |
| ------- | ------------------------------------- | --- | --- | --- |
C
| RLP  | Radio Link Protocol         |     |     |     |
| ---- | --------------------------- | --- | --- | --- |
| SIM  | Subscriber Identity Module  |     |     |     |
Terminal Adaptor, e.g. a GSM data card (equal to DCE; Data Circuit terminating
TA
Equipment)
TE  Terminal Equipment, e.g. a computer (equal to DTE; Data Terminal Equipment)
| UE    | User Equipment                        |     |     |     |
| ----- | ------------------------------------- | --- | --- | --- |
| UICC  | Universal Integrated Circuit Card     |     |     |     |
| USAT  | USIM Application Toolkit              |     |     |     |
| USIM  | Universal Subscriber Identity Module  |     |     |     |

RG50xQ&RM5xxQ_Series_Network_Application_Note                                136 / 136