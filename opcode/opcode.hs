module Main where

import Control.Monad (forM_)

data Opcode = BRK | JSR
    deriving (Enum, Bounded, Show)

data OpcodeInfo = OpcodeInfo {opcode :: Int, size :: Int}

opcodeInfo :: Opcode -> OpcodeInfo
opcodeInfo BRK = OpcodeInfo 0x00 1
opcodeInfo JSR = OpcodeInfo 0x20 3

main = forM_ [minBound .. maxBound] $ \opcode ->
    let OpcodeInfo code size = opcodeInfo opcode
     in putStrLn $
            show opcode ++ "=" ++ show code
                ++ " size="
                ++ show size
