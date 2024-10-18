import Image from "next/image";

export default function Technology() {
  return (
    <div className="px-4 max-w-lg mx-auto">
      <h1 className="text-2xl lg:text-3xl font-bold pt-10 text-left">
        Rugged Camera
      </h1>
      <p className="text-xl text-left mb-4">
        Our first tool is a rugged camera that can operate in harsh environments
        with minimal human interaction. More coming very soon.
      </p>
      <Image
        src="/camBoxV1.jpg"
        className="rounded-md mx-auto"
        alt="v1 camera box in Hawaii."
        width={500}
        height={500}
      />
    </div>
  );
}
