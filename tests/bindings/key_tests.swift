import unidemo

func testAll() throws {
    let separator = String(repeating: "🔮", count: 20)
    print("\n\(separator)\nHEY")
    let keyPair = KeyPair.zooWrong()
    let publicKey = keyPair.publicKey()
    print("Public Key: \(publicKey)")
    assert(publicKey.description == "0xfefd19b87ac6f83c8a3cc4e9603a00d4a9e6f8322a625786300380722550c47d")
}

func doTest() {
    do {
        try testAll()
    } catch {
        fatalError("Unexpected error: \(error)")
    }
}

doTest()