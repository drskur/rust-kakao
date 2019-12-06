# Rust Kakao
Kakao Rest API for Rust.

## Getting Started
1. Get Rest API KEY from [Kakao Developer](https://developers.kakao.com)
2. Set Dependency in cargo.toml
```toml
[dependencies]
kakao = { git = "https://github.com/rabyss/rust-kakao.git" }
```

## Usage
```rust
    let api_key = "your kakao rest api key"
    let client = KakaoLocal::new(&KakaoCred::new(&api_key));
    let output = client.search_address(&LocalSearchAddressInput {
        query: "전북 삼성로 100".to_string(),
        ..Default::default()
    })?;
```

## API coverages

- [ ] [UserManagement](https://developers.kakao.com/docs/restapi/user-management)
- [ ] [KakaoTalk](https://developers.kakao.com/docs/restapi/kakaotalk-api)
- [ ] [KakaoPay](https://developers.kakao.com/docs/restapi/kakaopay-api)
- [ ] [KakaoStory](https://developers.kakao.com/docs/restapi/kakaostory-api)
- [ ] [PushNotification](https://developers.kakao.com/docs/restapi/push-notification)
- [ ] [Search](https://developers.kakao.com/docs/restapi/search)
- [ ] [Local](https://developers.kakao.com/docs/restapi/local)
- [ ] [Vision](https://developers.kakao.com/docs/restapi/vision)
- [ ] [Translation](https://developers.kakao.com/docs/restapi/translation)
- [ ] [Speech](https://developers.kakao.com/docs/restapi/speech)

## Note
If you test library, you must set .env file.
```env
KAKAO_REST_API_KEY=your kakao rest api key
```

## License
MIT