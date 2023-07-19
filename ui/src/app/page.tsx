"use client";

import { useState } from "react";



const pieceMap = {
  1: "king",
  2: "queen",
  3: "rook",
  4: "knight",
  5: "bishop",
  6: "pawn",
};

const messageHandler = (message: any, socket: any) => {
  const [key, data] = message.split(":");

  switch (key) {
    case "id":
      sessionStorage.setItem("id", data);

      let payload = {
        id: data,
        action: "setup",
        ai: false,
      };

      let msg = JSON.stringify(payload);
      socket.send(msg);
      break;
  }
};

export default function Home() {
  let tempBoard: any = {};

  for (let i = 1; i <= 8; i++) {
    for (let j = 1; j <= 8; j++) {
      let key = `${i},${j}`;

      tempBoard[key] = {
        type: null,
        moves: [],
      };
    }
  }

  const [board, updateBoard] = useState(tempBoard);

  const socket = new WebSocket("ws://localhost:3030/game");

  socket.addEventListener("message", (e) => {
    messageHandler(e.data, socket);
  });

  // const socket = io("ws://localhost:3030", {
  //   path: "/game",
  // });
  // socket.emit("begin-game", (val: any) => {});

  return (
    <main
      className={`flex w-screen h-screen justify-center items-center`}
    >
      <div className="grid grid-rows-8 grid-flow-col gap-2">
        {Object.keys(board).map((key: any, index: any) => {
          let arr = key.split(",");

          let sum = arr.reduce((acc: any, val: any) => acc + parseInt(val), 0);

          let name = sum % 2 ? "bg-slate-800" : "bg-slate-200";

          return <div className={`${name} w-16 h-16`} key={key}></div>;
        })}
      </div>
    </main>
  );
}
