package cz.cvut.swa.bazaar.productservice.service

import com.fasterxml.jackson.databind.ObjectMapper
import cz.cvut.swa.bazaar.productservice.data.NetworkException
import cz.cvut.swa.bazaar.productservice.data.ProductRepository
import cz.cvut.swa.bazaar.productservice.data.ProductWithReviewsDTO
import cz.cvut.swa.bazaar.productservice.logger
import org.springframework.stereotype.Service

@Service
class ProductService(
        private val productRepository: ProductRepository,
        private val objectMapper: ObjectMapper,
        private val reviewService: ReviewService
) {

    private val log by logger()

    fun getProductWithReviews(productId: String): ProductWithReviewsDTO {
        log.debug("> getProductWithReviews - $productId")

        val product = productRepository.findById(productId)
                .orElseThrow { NoSuchElementException("Failed to find product") }

        val productWithReviewsDTO = objectMapper.convertValue(product, ProductWithReviewsDTO::class.java)
        val reviews = reviewService.fetchProductReviews(productId)
                ?: throw NetworkException("Failed to fetch reviews")

        productWithReviewsDTO.reviews = reviews

        log.debug("< getProductWithReviews - $productWithReviewsDTO")
        return productWithReviewsDTO
    }

}