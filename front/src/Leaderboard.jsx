import { createSignal, onMount } from "solid-js";

const Leaderboard = () => {
  const [leaderboard, setLeaderboard] = createSignal([]);
  const [page, setPage] = createSignal(1);
  const [totalPages, setTotalPages] = createSignal(1);
  const [wsConnection, setWSConnection] = createSignal(null);
  const [isWSConnected, setisWSConnected] = createSignal(false);

  // Fetch leaderboard data from the backend when the component mounts
  const connectWS = () => {
    
    fetchPageCount();

    // Listen updates from the WebSocket connection
    const ws = new WebSocket("ws://127.0.0.1:8080/ws/");
    setWSConnection(ws);

    ws.onmessage = (event) => {
      try {
        const updatedLeaderboard = JSON.parse(event.data);
        console.log("Leaderboard update:", updatedLeaderboard);
        setLeaderboard(updatedLeaderboard);
      } catch (error) {
        console.error("Error parsing leaderboard data:", error);
      }
    };

    ws.onopen = () => {
      setisWSConnected(true);
    };
    ws.onclose = () => {
      setisWSConnected(false);
    };

    return () => {
      ws.close();
      setisWSConnected(false);
    };
  };

  const fetchPageCount = async () => {
    try {
      const response = await fetch("http://127.0.0.1:8080/leaderboard_count");

      if (response.ok) {
        const data = await response.json();
        setTotalPages(Math.ceil(data / 5));
      }
    } catch (error) {
      console.error("Error fetching leaderboard count:", error);
    }
  };

  const fetchPage = async (page) => {
    try {
      const ws = wsConnection();
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.close();
        setisWSConnected(false);
        console.log("WebSocket connection closed");
      }

      const response = await fetch(`http://127.0.0.1:8080/leaderboard/${page}`);

      if (response.ok) {
        const data = await response.json();
        console.log("Leaderboard page:", page, data);
        setLeaderboard(data);
        setPage(page);
      }
    } catch (error) {
      console.error("Error fetching leaderboard data:", error);
    }
  };

  onMount(async () => {
    return connectWS();
  });

  return (
    <div class="max-w-xl mx-auto py-8">
      <h2 class="text-2xl font-bold text-center mb-6">LEAD</h2>
      <h4 class="text-lg font-bold text-center mb-6">High-Performance Leaderboard App</h4>
      <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
        <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
          <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
            <tr>
              <th scope="col" class="px-6 py-3">
                Name
              </th>
              <th scope="col" class="px-6 py-3 text-right">
                Score
              </th>
            </tr>
          </thead>
          <tbody>
            {leaderboard().length === 0 ? (
              <For each={[1, 2, 3, 4, 5]}>
                {(_) => (
                  <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                    <th
                      scope="row"
                      class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
                    >
                      <div class="h-2 bg-gray-200 rounded-full dark:bg-gray-700 max-w-[350px] mb-2.5"></div>
                    </th>
                    <td class="px-6 py-4">
                      <div class="h-2 bg-gray-200 rounded-full dark:bg-gray-700 max-w-[100px] mb-2.5 ml-auto"></div>
                    </td>
                  </tr>
                )}
              </For>
            ) : (
              <For each={leaderboard()}>
                {({ name, score }) => (
                  <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                    <th
                      scope="row"
                      class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
                    >
                      {name}
                    </th>
                    <td class="px-6 py-4 text-right">{score}</td>
                  </tr>
                )}
              </For>
            )}
          </tbody>
        </table>
      </div>

      <nav aria-label="pagination">
        <ul class="flex items-center h-8 text-sm justify-center mt-2">
          <li>
            <button
              disabled={page() === 1}
              onClick={() => fetchPage(page() - 1)}
              class="flex items-center justify-center px-3 h-8 ms-0 leading-tight text-gray-500 bg-white border border-e-0 border-gray-300 rounded-s-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
            >
              <span class="sr-only">Previous</span>
              <svg
                class="w-2.5 h-2.5 rtl:rotate-180"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 6 10"
              >
                <path
                  stroke="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M5 1 1 5l4 4"
                />
              </svg>
            </button>
          </li>
          {leaderboard().length === 0 ? (
            <li>
              <button
                disabled
                role="status"
                class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
              >
                <svg
                  aria-hidden="true"
                  class="w-6 h-6 text-gray-200 animate-spin dark:text-gray-600 fill-gray-300"
                  viewBox="0 0 100 101"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                    fill="currentColor"
                  />
                  <path
                    d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                    fill="currentFill"
                  />
                </svg>
                <span class="sr-only">Loading...</span>
              </button>
            </li>
          ) : (
            <>
              {page() - 1 > 0 && (
                <li>
                  <button
                    onClick={() => fetchPage(page() - 1)}
                    class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
                  >
                    {page() - 1}
                  </button>
                </li>
              )}

              <li>
                <button
                  disabled
                  class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
                >
                  {page()}
                </button>
              </li>

              {page() + 1 <= totalPages() && (
                <li>
                  <button
                    onClick={() => fetchPage(page() + 1)}
                    class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
                  >
                    {page() + 1}
                  </button>
                </li>
              )}
            </>
          )}

          <li>
            <button
              disabled={page() === totalPages()}
              onClick={() => fetchPage(page() + 1)}
              class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 rounded-e-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
            >
              <span class="sr-only">Next</span>
              <svg
                class="w-2.5 h-2.5 rtl:rotate-180"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 6 10"
              >
                <path
                  stroke="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="m1 9 4-4-4-4"
                />
              </svg>
            </button>
          </li>
          <li class="ml-auto">
            {isWSConnected() ? (
              <button class="px-3 py-1 text-xs font-medium leading-none text-center text-red-800 bg-white-200 rounded-full animate-pulse dark:bg-red-900 dark:text-blue-200">
                LIVE
              </button>
            ) : (
              <button
                onClick={() => connectWS()}
                class="px-3 py-1 text-xs font-medium leading-none text-center text-gray-500 bg-white border border-gray-300 rounded-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
              >
                SEE LIVE UPDATES
              </button>
            )}
          </li>
        </ul>
      </nav>
    </div>
  );
};

export default Leaderboard;
