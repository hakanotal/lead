import { createSignal, onMount } from "solid-js";

const Leaderboard = () => {
  // This signal will store the leaderboard data
  const [leaderboard, setLeaderboard] = createSignal([]);

  // Fetch leaderboard data from the backend when the component mounts
  onMount(async () => {
    const ws = new WebSocket("ws://127.0.0.1:8080/ws/");

    // Listen for messages from the WebSocket connection
    ws.onmessage = (event) => {
      // Assuming the message received is a JSON string of the updated leaderboard
      try {
        const updatedLeaderboard = JSON.parse(event.data);
        console.log("Leaderboard data received:", updatedLeaderboard);
        setLeaderboard(updatedLeaderboard);
      } catch (error) {
        console.error("Error parsing leaderboard data:", error);
      }
    };

    // Clean up on unmount
    return () => {
      ws.close();
    };
  });

  return (
    <div class="max-w-2xl mx-auto py-8">
      <h2 class="text-2xl font-bold text-center mb-6">Leaderboard</h2>

      <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
        <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
          <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
            <tr>
              <th scope="col" class="px-6 py-3">
                Name
              </th>
              <th scope="col" class="px-6 py-3">
                Score
              </th>
            </tr>
          </thead>
          <tbody>
            <For each={leaderboard()}>
              {({ name, score }) => (
                <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                  <th
                    scope="row"
                    class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
                  >
                    {name}
                  </th>
                  <td class="px-6 py-4">{score}</td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default Leaderboard;
