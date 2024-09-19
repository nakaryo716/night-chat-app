import React, { useState, useEffect, useRef } from 'react';
import { useLoaderData, useLocation } from 'react-router-dom';
import '../styles/Chat.css'; 
import { getRoomInfoApi } from '../api/roomApi';
import { RoomInfo } from '../types/room';

const Chat = () => {
  const [messages, setMessages] = useState<string[]>([]);
  const [roomName, setroomName] = useState("");
  const [input, setInput] = useState<string>('');
  const socketRef = useRef<WebSocket | null>(null);
  const location = useLocation();  
  const userName = useLoaderData();


  const removeTextNum = 6;
  const urlPath = location.pathname;
  const roomId = urlPath.substring(removeTextNum);

  useEffect(() => {
    const getRoomNameHandler = async () => {
      try {
        const response = await getRoomInfoApi(roomId);

        if (!response.ok) {
          throw new Error("error");
        }

        const roomInfo: RoomInfo = await response.json();
        setroomName(roomInfo.room_name);
      } catch {
        console.error("failed to get room name");
        setroomName("unkown");
      }
    }  
    getRoomNameHandler();
  }, [roomId]);



  useEffect(() => {
    const websocket = new WebSocket(`ws://localhost:3000/websocket/${roomId}?user_name=${userName}`);
    socketRef.current = websocket;

    const onMessage = (event: MessageEvent<string>) => {
      setMessages((prevMessages) => [...prevMessages, event.data]);
    };

    websocket.addEventListener('message', onMessage);

    return () => {
      console.log("websocket closed");
      websocket.removeEventListener('message', onMessage);
      websocket.close();
    };
  }, [roomId, userName]); 

  const handleSend = () => {
    if (input.trim() === '') return;
    socketRef.current?.send(input);
    setInput('');
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && e.altKey) {
      handleSend();
    }
  };

  return (
    <>
      <h1 className='roomName'>{roomName}</h1>
      <div className="chatContainer">
        <div className="messageContainer">
          {messages.map((message, index) => (
            <div key={index} className="message">
              <p>{message}</p>
            </div>
          ))}
        </div>
      </div>
      <div className='inputContainer'>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="input"
          placeholder="Alt(Option) + Enterで送信"
          onKeyDown={handleKeyDown}
        />
        <button onClick={handleSend} className="sendButton" type='submit'>
          送信
        </button>
      </div>
    </>
  );
};

export default Chat;
