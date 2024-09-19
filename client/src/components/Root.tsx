import {useNavigate } from 'react-router-dom';
import '../styles/Root.css'; // 外部CSSをインポート

export const Root = () => {
    const navigate = useNavigate();

    const handleJoinChat = () => {
        navigate("/room-list");
    }

    const handleSetUserName = () => {
        navigate("/user-name");
    }

    const handleCreateRoom = () => {
        navigate("/create-room");
    }
    return (
        <div className="container">
            <div className="content">
                <h1 className="mainTitle">夜のひととき、深まるつながり</h1>
                <h2 className="subTitle">夜だけ使えるチャットアプリ</h2>
                <p className="description"><strong>18時から24時</strong>までの間だけ使えるチャットアプリです。</p>
                <p className="description">ユーザー登録なしで、すぐに参加できます。</p>
                <h2 className="subTitle">時間制限がある理由</h2>
                <p className="description">
                    時間制限があるからこそ、人とのつながりをより大切にし、密接な会話を楽しんでほしいと考えています。
                </p>
            </div>
            <div className="buttonContainer">
                <button className="customButton1" onClick={handleSetUserName}>ユーザー名を決める</button>
                <button className="customButton2" onClick={handleJoinChat}>チャットに参加する</button>
                <button className="customButton3" onClick={handleCreateRoom}>チャットルームを作る</button>
            </div>
        </div>
    );
}
