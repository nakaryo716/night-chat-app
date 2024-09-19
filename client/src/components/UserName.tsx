import { useState } from "react";
import "../styles/UserName.css";
import { setUserNameApi } from "../api/userApi";
import { User } from "../types/user";
import { useNavigate } from "react-router-dom";
export const UserName = () => {
    const [input, setInput] = useState("");
    const [error, setError] = useState("");
    const navigate = useNavigate();

    const setUserNameHandler = async () => {
        if (!input) {
            alert("ユーザー名を入力してください");
            return;
        }

        const newUserpayload: User = {
            user_name: input,
        }

        try {
            const res = await setUserNameApi(newUserpayload);
            if(!res.ok) {
                throw new Error("set user name error");
            }
            alert("ユーザー名を変更しました");
            navigate("/");
        } catch {
            setError("ユーザー名の変更に失敗しました");
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
            <div className="userNameContainer">
                <h1>ユーザー名を決める</h1>
                <div>
                    <input  
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        className="inputTxt"
                        placeholder="あなたのお名前"
                        onKeyDown={keyDownHandle}
                    />
                    <button className="customButton" onClick={setUserNameHandler}> 送信</button>
                </div>
            </div>
        </>
    );
}
