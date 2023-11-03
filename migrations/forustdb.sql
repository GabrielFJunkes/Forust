CREATE TABLE IF NOT EXISTS `inscricoes` (
  `user_id` integer NOT NULL,
  `comunidade_id` integer NOT NULL,
  `admin` boolean DEFAULT FALSE,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS `usuarios` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
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
  `user_id` integer,
  `comunidade_id` integer,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS `usuarios_avaliam_posts` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `post_id` integer,
  `usuario_id` integer,
  `gostou` boolean NOT NULL
);

CREATE TABLE IF NOT EXISTS `usuarios_avaliam_comentarios` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `comentario_id` integer,
  `usuario_id` integer,
  `gostou` boolean NOT NULL
);

CREATE TABLE IF NOT EXISTS `posts_tem_tags` (
  `post_id` integer,
  `tag_id` integer
);

CREATE TABLE IF NOT EXISTS `comentarios` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `body` text NOT NULL,
  `post_id` integer,
  `usuario_id` integer,
  `comentario_id` integer
);

ALTER TABLE `inscricoes` ADD CONSTRAINT PRIMARY KEY(`user_id`, `comunidade_id`);

ALTER TABLE `posts_tem_tags` ADD CONSTRAINT PRIMARY KEY(`post_id`, `tag_id`);

ALTER TABLE `inscricoes` ADD FOREIGN KEY (`user_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `inscricoes` ADD FOREIGN KEY (`comunidade_id`) REFERENCES `comunidades` (`id`);

ALTER TABLE `tags` ADD FOREIGN KEY (`comunidade_id`) REFERENCES `comunidades` (`id`);

ALTER TABLE `posts` ADD FOREIGN KEY (`user_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `posts` ADD FOREIGN KEY (`comunidade_id`) REFERENCES `comunidades` (`id`);

ALTER TABLE `usuarios_avaliam_posts` ADD FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`);

ALTER TABLE `usuarios_avaliam_posts` ADD FOREIGN KEY (`usuario_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `usuarios_avaliam_comentarios` ADD FOREIGN KEY (`comentario_id`) REFERENCES `comentarios` (`id`);

ALTER TABLE `usuarios_avaliam_comentarios` ADD FOREIGN KEY (`usuario_id`) REFERENCES `usuarios` (`id`);

ALTER TABLE `posts_tem_tags` ADD FOREIGN KEY (`tag_id`) REFERENCES `tags` (`id`);

ALTER TABLE `posts_tem_tags` ADD FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`);

ALTER TABLE `comentarios` ADD FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`);

ALTER TABLE `comentarios` ADD FOREIGN KEY (`comentario_id`) REFERENCES `comentarios` (`id`);
