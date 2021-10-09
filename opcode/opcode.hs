module Main where

import Control.Monad (forM_)

data Op = BRK | JSR
  deriving (Enum, Bounded, Show)

data Info = Info
  { code :: Int
  , size :: Int
  }

opcodeInfo :: Op -> Info
opcodeInfo BRK = Info 0x00 1
opcodeInfo JSR = Info 0x20 3

main = forM_ [minBound .. maxBound] $ \opcode ->
  let i = opcodeInfo opcode
   in putStrLn $
        show opcode ++ "=" ++ show (code i)
          ++ " size="
          ++ show (size i)
