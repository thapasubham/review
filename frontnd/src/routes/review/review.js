export async function fetchMovieDetails(id) {
    try {
      const response = await fetch(`http://127.0.0.1:3000/review/${id}`);
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      const data = await response.json();
      return data; 
    } catch (error) {
      console.error('Error fetching movie details:', error);
      throw error; // Rethrow the error for handling in the component
    }
  }

  export async function fetchReviews(id){
    try {
        const response = await fetch(`http://127.0.0.1:3000/review/${id}/review`);
        if (!response.ok) {
          throw new Error(`HTTP error! Status: ${response.status}`);
        }
        const review = await response.json();
        return review; 
      } catch (error) {
        console.error('Error fetching movie details:', error);
        throw error; // Rethrow the error for handling in the component
      }
  }