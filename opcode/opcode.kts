enum class Opcode(val opcode: Int, val size: Int) {
  BRK(0x00, 1),
  JSR(0x20, 3)
}

for (opcode in Opcode.values()) {
  println("${opcode.name}=${opcode.opcode} (size ${opcode.size})")
}
