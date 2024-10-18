import Link from "next/link";
import Image from "next/image";

export default function Navbar() {
  return (
    <div>
      <Link href="/" className="hover:cursor-pointer">
        <Image
          src="/logoBeakerTree.png"
          alt="Adventure Labs"
          width={60}
          height={60}
          className="fixed top-10 left-6 z-20 invisible lg:visible"
        />
      </Link>
      <div className="ring ring-1 fixed top-10 right-10 rounded-md text-xl flex flex-row z-20 bg-gray-400/50 backdrop-blur-md">
        <Link
          href="/"
          className={`hover:cursor-pointer hover:bg-primary/20 px-4 py-2 rounded-tr-md rounded-br-md transition-colors duration-300`}
        >
          Home
        </Link>
        <Link
          href="/about"
          className={`hover:cursor-pointer hover:bg-primary/20 px-4 py-2 rounded-tr-md rounded-br-md transition-colors duration-300`}
        >
          About
        </Link>
        <Link
          href="/technology"
          className={`hover:cursor-pointer hover:bg-primary/20 px-4 py-2 rounded-tr-md rounded-br-md transition-colors duration-300`}
        >
          Technology
        </Link>
      </div>
    </div>
  );
}
