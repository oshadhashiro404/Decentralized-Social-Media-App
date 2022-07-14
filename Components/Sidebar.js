import Image from 'next/image';
import { sidebarData } from '../static/static';
import RightSidebar from './RightSidebar';

const Sidebar = ({ name, url, setUsers, users, getUsers }) => {
	const style = {
		wrapper: `py-[25px] px-[10px] w-[24rem] `,
		divider: `w-[95%] border-b border-[0.5px] border-[#3e4042] my-2`,
		sidebarRow: `flex w-full mb-[20px] hover:bg-[#2a2b2c] transition-all duration-300 ease-in-out rounded-lg p-[5px] gap-[10px] cursor-pointer`,
		profileImage: `rounded-full object-cover`,
		sidebarItem: `text-white font-semibold flex items-left  flex-col justify-center text-sm `,
	};
	return (
		<div className={style.wrapper}>
			<div className={style.sidebarRow}>
				<Image
					className={style.profileImage}
					src={url}
					height={30}
					width={30}
					alt="profile image"
				/>
				<div className={style.sidebarItem}>{name}</div>
			</div>
			{sidebarData.map((sidebarDataItem, index) => (
				<div className={style.sidebarRow} key={index}>
					<Image
						className={style.sidebarIcon}
						src={sidebarDataItem.icon}
						height={30}
						width={30}
						alt="sidebar icon"
					/>
					<div className={style.sidebarItem}>{sidebarDataItem.title}</div>
				</div>
			))}

			<div className={style.divider} />

			<RightSidebar getUsers={getUsers} users={users} setUsers={setUsers} />
		</div>
	);
};

export default Sidebar;
