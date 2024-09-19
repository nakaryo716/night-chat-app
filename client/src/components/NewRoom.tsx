import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { CreateRoom, RoomInfo } from "../types/room";
import { createRoomApi } from "../api/roomApi";
import "../styles/NewRoom.css"
export const NewRoom = () => {
    const [input, setInput] = useState("");
    const [error, setError] = useState("");
    const navigate = useNavigate();

    const setUserNameHandler = async () => {
        if (!input) {
            alert("ルーム名を入力してください");
            return;
        }

        const newRoomPayload: CreateRoom = {
            room_name: input,
        };

        try {
            const res = await createRoomApi(newRoomPayload);
            if(!res.ok) {
                throw new Error("set user name error");
            }

            const room: RoomInfo = await res.json();
            const roomId = room.room_id;
            navigate(`/chat/${roomId}`);
        } catch {
            setError("チャットルームの作成に失敗しました");
        }
    }

    const keyDownHandle = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === "Enter") {
            setUserNameHandler();
        }
    };
    
    if (error) {
        return(
            <h1 style={{color: "red"}}>{error}</h1>
        );
    }

    return (
        <>
            <div className="userRoomContainer">
                <h1>チャットルームを作成する</h1>
                <div>
                    <input  
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        className="inputTxt"
                        placeholder="チャットルーム名"
                        onKeyDown={keyDownHandle}
                    />
                    <button className="customButtonRoom" onClick={setUserNameHandler}> 送信</button>
                </div>
            </div>
        </>
    );
}


