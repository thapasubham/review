-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Sep 22, 2024 at 03:40 PM
-- Server version: 10.4.27-MariaDB
-- PHP Version: 8.1.12

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `movie_review`
--

-- --------------------------------------------------------

--
-- Table structure for table `admin`
--

CREATE TABLE `admin` (
  `admin_id` int(11) NOT NULL,
  `username` varchar(40) NOT NULL,
  `password` varchar(40) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `admin`
--

INSERT INTO `admin` (`admin_id`, `username`, `password`) VALUES
(1, 'subham22', 'Subham223');

-- --------------------------------------------------------

--
-- Table structure for table `members`
--

CREATE TABLE `members` (
  `m_id` int(11) NOT NULL,
  `firstname` varchar(30) NOT NULL,
  `lastname` varchar(20) NOT NULL,
  `username` varchar(30) NOT NULL,
  `phone` varchar(10) NOT NULL,
  `email` varchar(40) NOT NULL,
  `password` varchar(40) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `members`
--

INSERT INTO `members` (`m_id`, `firstname`, `lastname`, `username`, `phone`, `email`, `password`) VALUES
(18, 'Bardan', 'Karki', 'bardan23', '9824537695', 'bardankarki23@gmail.com', 'Bardan43'),
(20, 'Fish', 'Stick', 'fortnite_21', '4897379490', 'fishstick@salty.com', 'Final Change23'),
(21, 'Gan', 'DE', 'subham', '0123456789', 'abc@gmail.com', 'Abc123456'),
(22, 'ABC', 'DE', 'valki', '0123456789', 'funk@gmail.com', 'Fortnie_jon13'),
(23, 'Sunil', 'Gamla', 'gamal_hamal', '9751642830', 'gamal23@yep.com', 'Gamal23.1'),
(24, 'ABC', 'DE', 'abc_dc', '9854587512', 'abc@gmail.com', 'Abc1234.'),
(25, 'FIzoj ', 'Maharrjan', 'firoz112', '2514527650', 'firoz@gmail.com', 'Firoj_123'),
(29, 'Test', 'Test', 'test123', '1234567890', 'test@gmail.com', '@Sub123'),
(30, 'Firoz', 'Maharjan', 'firozmaharjan', '9812345744', 'abcd@gmail.com', '12345678Aa'),
(31, 'Agent', 'Jonsey', 'Jonsey', '9578124850', 'jordan@gmail.com', '1234Hello'),
(32, 'Arnav', 'Poudel', 'arnav', '9751678548', 'arnav@gmail.com', 'Arnav.123'),
(36, 'Subham', 'Thapa', 'subham3', '7854962153', 'subham@gmail.com', 'Subham3.');

-- --------------------------------------------------------

--
-- Table structure for table `movie`
--

CREATE TABLE `movie` (
  `movie_id` int(11) NOT NULL,
  `img_name` varchar(255) NOT NULL,
  `movie_name` varchar(50) NOT NULL,
  `released` int(4) NOT NULL,
  `about_movie` text NOT NULL,
  `genre` varchar(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `movie`
--

INSERT INTO `movie` (`movie_id`, `img_name`, `movie_name`, `released`, `about_movie`, `genre`) VALUES
(1, 'Lord Of the Rings.jpg', 'Lord Of the Rings', 2003, 'A battle between good and evil in which a Hobbit must deliver a ring into a volcano.', 'Action'),
(2, 'Drive.jpeg', 'Drive', 2011, 'A getaway driver falls in love with Irene, a criminal\'s wife. He gets involved in a robbery attempt and lands himself in trouble with the mob to protect his lover.', 'Crime'),
(3, 'John Wick.jpg', 'John Wick', 2018, 'An ex-hitman comes out of retirement to track down the gangsters who killed his dog and stole his car.', 'Action'),
(4, 'The Place Beyond The Pines.jpg', 'The Place Beyond The Pines', 2013, 'A motorcycle stunt rider turns to robbing banks as a way to provide for his ex and their newborn', 'Crime'),
(5, 'pacificRim.jpg', 'Pacific Rim', 2013, 'As a war between humankind and monstrous sea creatures wages on, a former pilot and a trainee are paired up to drive a seemingly obsolete special weapon in a desperate effort to save the world from the apocalypse', 'Si-Fi'),
(6, 'Star Wars.png', 'Star Wars', 2019, ' A young jedi assemeble the jedi order with the help of the friend.', 'History'),
(7, 'Spider Man.jpg', 'Spider Man', 2020, 'Miles Morales gains the ability of a spider and becomes his local town hero.', 'History'),
(9, 'Freiren.png', 'Freiren', 2024, 'Elf girl.', 'Comedy'),
(20, 'Spiderman.png', 'Spiderman', 0, 'fdjn', 'Comedy');

-- --------------------------------------------------------

--
-- Table structure for table `review`
--

CREATE TABLE `review` (
  `review_id` int(11) NOT NULL,
  `reviewed_by` int(11) NOT NULL,
  `movie_id` int(11) NOT NULL,
  `review_date` date DEFAULT curdate(),
  `review_msg` text NOT NULL,
  `star` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `review`
--

INSERT INTO `review` (`review_id`, `reviewed_by`, `movie_id`, `review_date`, `review_msg`, `star`) VALUES
(5, 22, 2, '2024-05-21', 'Absolutely amazing i can’t believe i put off watching this movie for years. Exciting right form the beginning too the very end.', 4),
(6, 22, 1, '2024-05-21', 'I absolutely adore this movie', 5),
(7, 18, 1, '2024-06-23', 'Loved it from start to end', 2),
(14, 20, 5, '2024-06-24', 'I completely understand that this is fake', 2),
(15, 29, 3, '2024-06-25', 'Nice john wick. I love you', 5),
(16, 20, 3, '2024-06-05', 'One of the best action flim I ve ever watched', 4),
(17, 30, 2, '2024-06-06', 'nice movie', 5),
(18, 20, 6, '2024-06-06', 'Boring', 1),
(19, 20, 2, '2024-06-06', 'Nice movie I love it.', 3),
(24, 24, 3, '2024-09-14', '6', 5),
(28, 29, 2, '2024-09-14', 'Very Good', 3);

--
-- Indexes for dumped tables
--

--
-- Indexes for table `admin`
--
ALTER TABLE `admin`
  ADD PRIMARY KEY (`admin_id`);

--
-- Indexes for table `members`
--
ALTER TABLE `members`
  ADD PRIMARY KEY (`m_id`),
  ADD UNIQUE KEY `username` (`username`);

--
-- Indexes for table `movie`
--
ALTER TABLE `movie`
  ADD PRIMARY KEY (`movie_id`);

--
-- Indexes for table `review`
--
ALTER TABLE `review`
  ADD PRIMARY KEY (`review_id`),
  ADD KEY `reviewed_by` (`reviewed_by`),
  ADD KEY `movie_id` (`movie_id`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `admin`
--
ALTER TABLE `admin`
  MODIFY `admin_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=2;

--
-- AUTO_INCREMENT for table `members`
--
ALTER TABLE `members`
  MODIFY `m_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=37;

--
-- AUTO_INCREMENT for table `movie`
--
ALTER TABLE `movie`
  MODIFY `movie_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=22;

--
-- AUTO_INCREMENT for table `review`
--
ALTER TABLE `review`
  MODIFY `review_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=29;

--
-- Constraints for dumped tables
--

--
-- Constraints for table `review`
--
ALTER TABLE `review`
  ADD CONSTRAINT `movie_id` FOREIGN KEY (`movie_id`) REFERENCES `movie` (`movie_id`),
  ADD CONSTRAINT `reviewed_by` FOREIGN KEY (`reviewed_by`) REFERENCES `members` (`m_id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
