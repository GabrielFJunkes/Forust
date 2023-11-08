-- Add migration script here
CREATE TABLE IF NOT EXISTS `inscricoes` (
  `usuario_id` integer PRIMARY KEY,
  `comunidade_id` integer PRIMARY KEY,
  `admin` boolean DEFAULT FALSE,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS `usuarios` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL UNIQUE,
  `email` varchar(255) NOT NULL UNIQUE,
  `senha` varchar(255) NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS `comunidades` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `nome` varchar(255) UNIQUE NOT NULL,
  `desc` text NOT NULL
);

CREATE TABLE IF NOT EXISTS `tags` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `nome` varchar(255),
  `status` boolean NOT NULL DEFAULT TRUE,
  `comunidade_id` integer NOT NULL
);

CREATE TABLE IF NOT EXISTS `posts` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `titulo` varchar(255) NOT NULL,
  `body` text NOT NULL,
  `tag_id` integer,
  `usuario_id` integer NOT NULL,
  `comunidade_id` integer NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS `usuarios_avaliam_posts` (
  `post_id` integer PRIMARY KEY,
  `usuario_id` integer PRIMARY KEY,
  `gostou` boolean NOT NULL
);

CREATE TABLE IF NOT EXISTS `usuarios_avaliam_comentarios` (
  `comentario_id` integer PRIMARY KEY,
  `usuario_id` integer PRIMARY KEY,
  `gostou` boolean NOT NULL
);

CREATE TABLE IF NOT EXISTS `comentarios` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `body` text NOT NULL,
  `post_id` integer NOT NULL,
  `usuario_id` integer NOT NULL,
  `comentario_id` integer
);

ALTER TABLE `inscricoes` ADD CONSTRAINT PRIMARY KEY(`usuario_id`, `comunidade_id`);

ALTER TABLE `inscricoes` ADD FOREIGN KEY (`usuario_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `inscricoes` ADD FOREIGN KEY (`comunidade_id`) REFERENCES `comunidades` (`id`);

ALTER TABLE `tags` ADD FOREIGN KEY (`comunidade_id`) REFERENCES `comunidades` (`id`);

ALTER TABLE `posts` ADD FOREIGN KEY (`usuario_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `posts` ADD FOREIGN KEY (`tag_id`) REFERENCES `tags` (`id`);

ALTER TABLE `posts` ADD FOREIGN KEY (`comunidade_id`) REFERENCES `comunidades` (`id`);

ALTER TABLE `usuarios_avaliam_posts` ADD FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`);

ALTER TABLE `usuarios_avaliam_posts` ADD FOREIGN KEY (`usuario_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `usuarios_avaliam_comentarios` ADD FOREIGN KEY (`comentario_id`) REFERENCES `comentarios` (`id`);

ALTER TABLE `usuarios_avaliam_comentarios` ADD FOREIGN KEY (`usuario_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `comentarios` ADD FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`);

ALTER TABLE `comentarios` ADD FOREIGN KEY (`comentario_id`) REFERENCES `comentarios` (`id`);
