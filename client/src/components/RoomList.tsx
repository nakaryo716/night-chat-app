import React, { useState, useEffect } from 'react';
import { RoomInfo } from '../types/room';
import { getRoomsApi } from '../api/roomApi';
import '../styles/RoomList.css';

export const RoomList: React.FC = () => {
  const [rooms, setRooms] = useState<RoomInfo[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchRooms = async () => {
        try {
            const response = await getRoomsApi();

            if (!response.ok) {
                const statusCode = response.status;
                if (statusCode === 403) {
                  setError("現在は時間外のためお休み中zzz");  // 403エラーの場合
                } else {
                  setError("取得エラーが発生しました。");  // その他のエラー
                }
                return;
            }
            const data: RoomInfo[] = await response.json();
            setRooms(data);
          } catch  (err) {
            console.log(err);
            setError("取得エラーが発生しました。");  // API呼び出し自体が失敗した場合
        }
    };
    fetchRooms();
  }, []);

  if (error) {
    if (error === "現在は時間外のためお休み中zzzまた明日") {
      return <h2 className='outOfTime'>{error}</h2>
    } else {
      return <h2 className='error'>{error}</h2>
    }
  }

  return (
    <>
    <h1 className='title'>チャットルーム一覧</h1>
    <div className='roomListContainer'>
      <div className='header'>
      </div>
      <ul className='roomList'>
        {rooms.map((room) => (
          <li className='roomItem' key={room.room_id}>
            <a className='roomLink' href={`/chat/${room.room_id}`}>
              {room.room_name}
            </a>
          </li>
        ))}
      </ul>
    </div>
    </>
  );
};
