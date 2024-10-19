export default function Home() {
  return (
    <div className="px-4 py-4 h-[100vh] w-[100vw] bg-quaternary absolute top-0 left-0">
      <div className="h-full w-full mx-auto my-auto rounded-xl bg-green-400/20 ring ring-3 ring-primary ring-quaternary bg-gradient-to-r from-primary to-secondary via-tertiary">
        <div className="mx-auto w-fit">
          <h1 className="text-4xl lg:text-7xl font-bold text-center pt-[34vh]">
            Adventure Labs
          </h1>
          <p className="lg:text-2xl text-center text-sm">
            Keeping a pulse on Earth's heartbeat
          </p>
        </div>
      </div>
    </div>
  );
}
