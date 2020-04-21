package cz.cvut.swa.bazaar.productservice.controller

import ReviewDTO
import com.fasterxml.jackson.databind.ObjectMapper
import cz.cvut.swa.bazaar.productservice.data.ProductRepository
import cz.cvut.swa.bazaar.productservice.data.Review
import cz.cvut.swa.bazaar.productservice.data.ReviewRepository
import cz.cvut.swa.bazaar.productservice.logger
import org.springframework.http.HttpStatus
import org.springframework.web.bind.annotation.*

@RestController
@RequestMapping("review")
class ReviewController(
        val objectMapper: ObjectMapper,
        val productRepository: ProductRepository,
        val reviewRepository: ReviewRepository
) {

    val log by logger()

    @PostMapping("/{productId}")
    @ResponseStatus(HttpStatus.OK)
    fun reviewProduct(@PathVariable(name = "productId") productId: String, @RequestBody reviewDTO: ReviewDTO): Review {
        log.debug("> reviewProduct - $productId")

        val product = productRepository.findById(productId).orElseThrow()

        log.debug("Creating review...")

        val review = objectMapper.convertValue(reviewDTO, Review::class.java)
        review.productId = product.id
        reviewRepository.save(review)

        log.debug("< reviewProduct - $review")
        return review
    }

    @PostMapping("/update")
    @ResponseStatus(HttpStatus.OK)
    fun updateReview(@RequestBody review: Review): Review {
        log.debug("> updateReview - $review")

        val updatedReview = reviewRepository.save(review)

        log.debug("< updateReview - $updatedReview")
        return updatedReview
    }

    @DeleteMapping("/{id}")
    @ResponseStatus(HttpStatus.OK)
    fun deleteReview(@PathVariable(name = "id") id: String) {
        log.debug("> deleteReview - $id")

        reviewRepository.deleteById(id)
    }

}