import React, { useState, useEffect, useRef } from 'react';
import { useLoaderData, useLocation } from 'react-router-dom';
import '../styles/Chat.css'; 
import { getRoomInfoApi } from '../api/roomApi';
import { RoomInfo } from '../types/room';
import { ChatMessage } from '../types/chat';

const Chat = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
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
      const data: ChatMessage= JSON.parse(event.data);
      setMessages((prevMessages) => [...prevMessages, data]);
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

  const formatTime = (timestamp: Date) => {
    const date = new Date(timestamp);
    return `${date.getFullYear()}/${date.getMonth()}/${date.getDate()} ${date.getHours()}:${date.getMinutes().toString().padStart(2, '0')}:${date.getSeconds().toString().padStart(2, '0')}`;
  };

  return (
    <>
      <h1 className='roomName'>{roomName}</h1>
      <div className="chatContainer">
        <div className="messageContainer">
          {messages.map((message, index) => (
            <div key={index} className="message">
              <div className='messageMetaData'>
                <p className='messageUserName'>{message.user_name}</p>
                <p className='messageTime'>{formatTime(message.time_stamp)}</p>
              </div>
              <div className='messageBody'>
                <p className='messageBodyText'>{message.text}</p>
              </div>
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
