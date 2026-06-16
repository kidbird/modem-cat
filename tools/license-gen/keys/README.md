# 私钥目录

此目录存放 License 签名私钥 `modem-cat.sk`。

## 获取私钥

私钥应从 modem-cat 项目根目录的 `keys/` 目录复制：

```bash
cp d:/code/modem-cat/keys/modem-cat.sk ./modem-cat.sk
```

## 其他部署方式

1. **开发模式**：将私钥放在 `keys/modem-cat.sk`（即本目录）
2. **生产模式**：将私钥放在 `<exe_dir>/keys/modem-cat.sk`（与 license-gen.exe 同级）
3. **环境变量**：设置 `MODEM_CAT_SK_PATH` 指向私钥文件路径

## 安全提示

- 私钥用于生成 License 文件，请妥善保管
- 不要将私钥提交到版本控制系统
- 生产环境建议使用独立密钥对
