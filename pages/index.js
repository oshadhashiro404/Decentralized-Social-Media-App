import { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import SignUp from '../Components/SignUp.js';
import Header from '../Components/Header.js';
import Feed from '../Components/Feed.js';
import RightSidebar from '../Components/RightSidebar.js';
import Sidebar from '../Components/Sidebar.js';

const style = {
	wrapper: `bg-[#18191a] min-h-screen duration-[0.5s]`,
	homeWrapper: `flex`,
	center: `flex-1`,
	main: `flex-1 flex justify-center`,
	signupContainer: `flex items-center justify-center w-screen h-[70vh]`,
};

export default function Home() {
	const [registered, setRegistered] = useState(false);
	const [name, setName] = useState();
	const [url, setUrl] = useState();
	const [users, setUsers] = useState([]);

	useEffect(() => {
		(async () => {
			await requestUsersData();
		})();
	}, []);

	const wallet = useWallet();

	const requestUsersData = async (activeAccount) => {
		try {
			const response = await fetch(`/api/fetchUsers`);
			const data = await response.json();

			setUsers(data.data);
		} catch (error) {
			console.error(error);
		}
	};

	return (
		<div className={style.wrapper}>
			<Header name={name} url={url} />

			{registered ? (
				<div className={style.homeWrapper}>
					{/* <Sidebar name={name} url={url} /> */}
					<div className={style.main}>
						<Feed connected={wallet.connected} name={name} url={url} />
					</div>

					<Sidebar
						name={name}
						url={url}
						users={users}
						setUsers={setUsers}
						getUsers={requestUsersData}
					/>
					{/* <RightSidebar
						getUsers={requestUsersData}
						users={users}
						setUsers={setUsers}
					/> */}
				</div>
			) : (
				<div className={style.signupContainer}>
					<SignUp
						setRegistered={setRegistered}
						name={name}
						setName={setName}
						url={url}
						setUrl={setUrl}
					/>
				</div>
			)}
		</div>
	);
}
